use std::sync::Arc;
use std::time::Duration;

use async_std::task::spawn;
use crabgrab::feature::wgpu::WgpuCaptureConfigExt as _;
use crabgrab::feature::wgpu::WgpuVideoFrameExt as _;
use crabgrab::prelude::*;
use futures::executor::block_on;

#[allow(unused)]
struct Gfx {
    device: wgpu::Device,
    queue: wgpu::Queue,
}

impl AsRef<wgpu::Device> for Gfx {
    fn as_ref(&self) -> &wgpu::Device {
        &self.device
    }
}

fn main() {
    block_on(async {
        let token = match CaptureStream::test_access(false) {
            Some(token) => token,
            None => CaptureStream::request_access(false)
                .await
                .expect("Expected capture access"),
        };
        let wgpu_instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            #[cfg(target_os = "windows")]
            backends: wgpu::Backends::DX12,
            #[cfg(target_os = "macos")]
            backends: wgpu::Backends::METAL,
            flags: wgpu::InstanceFlags::VALIDATION | wgpu::InstanceFlags::GPU_BASED_VALIDATION,
            dx12_shader_compiler: wgpu::Dx12Compiler::default(),
            gles_minor_version: wgpu::Gles3MinorVersion::default(),
        });
        let wgpu_adapter = wgpu_instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                force_fallback_adapter: false,
                compatible_surface: None,
            })
            .await
            .expect("Expected wgpu adapter");
        let (wgpu_device, wgpu_queue) = wgpu_adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: Some("wgpu adapter"),
                    required_features: wgpu::Features::default(),
                    required_limits: wgpu::Limits::default(),
                },
                None,
            )
            .await
            .expect("Expected wgpu device");
        let gfx = Arc::new(Gfx {
            device: wgpu_device,
            queue: wgpu_queue,
        });
        let filter = CapturableContentFilter::DISPLAYS;
        let content = CapturableContent::new(filter)
            .await
            .expect("Expected to get capturable displays");
        let display = content
            .displays()
            .next()
            .expect("Expected at least one capturable display");

        let window = content
            .windows()
            .filter(|window| {
                let app_identifier = window.application().identifier();
                window.title().len() != 0 && app_identifier.to_lowercase().contains("chrome")
            })
            .next()
            .unwrap();
        //let config = CaptureConfig::with_display(display, CapturePixelFormat::Bgra8888)
        let config = CaptureConfig::with_window(window, CapturePixelFormat::Bgra8888)
            .unwrap()
            .with_wgpu_device(gfx.clone())
            .expect("Expected config with wgpu device");
        let (tx_result, mut rx_result) =
            futures::channel::oneshot::channel::<Result<Option<VideoFrame>, StreamError>>();
        let mut tx_result = Some(tx_result);
        let _stream = CaptureStream::new(token, config, move |event_result| match event_result {
            Ok(event) => match event {
                StreamEvent::Video(frame) => {
                    if let Some(tx_result) = tx_result.take() {
                        println!("Sending frame...");
                        tx_result
                            .send(Ok(Some(frame)))
                            .expect("Expected to send result");
                    }
                }
                StreamEvent::End => {
                    if let Some(tx_result) = tx_result.take() {
                        tx_result.send(Ok(None)).expect("Expected to send result");
                    }
                }
                _ => {}
            },
            Err(error) => {
                if let Some(tx_result) = tx_result.take() {
                    tx_result.send(Err(error)).expect("Expected to send result");
                }
            }
        })
        .expect("Expected capture stream");
        println!("Stream started. Awaiting message...");

        std::thread::spawn(move || loop {
            if let res = rx_result.try_recv() {
                match res {
                    Ok(res) => match res {
                        Some(res) => match res {
                            Ok(res) => match res {
                                Some(frame) => {
                                    println!("Got frame! getting wgpu texture...");
                                    let wgpu_texture = frame
                    .get_wgpu_texture(
                        crabgrab::feature::wgpu::WgpuVideoFramePlaneTexture::Rgba,
                        Some("wgpu video frame"),
                    )
                    .expect("Expected wgpu texture from video frame");
                                    println!(
                                        "Got wgpu texture! Size: {:?}, Format: {:?}",
                                        wgpu_texture.size(),
                                        wgpu_texture.format()
                                    );
                                }
                                None => println!("err 1"),
                            },
                            Err(err) => println!("err 2 :{:?}", err),
                        },
                        None => println!("err 3"),
                    },
                    Err(err) => println!("err 4: {:?}", err),
                }
            }
        });
        std::thread::sleep(Duration::from_millis(10000));
    });
}
