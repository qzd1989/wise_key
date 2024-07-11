use crate::common::capture_send;
use crabgrab::{feature::bitmap::VideoFrameBitmap as _, prelude::*};
use image::{DynamicImage, ImageBuffer, Rgba};
use std::sync::Arc;
use std::{thread::sleep, time::Duration};
use wgpu::Texture;

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

pub fn run() {
    let runtime = tokio::runtime::Builder::new_multi_thread().build().unwrap();
    let future = runtime.spawn(async {
        //gpu init
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
            let mut wgpu_encoder = wgpu_device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("encoder"),
            });
            let wgpu_buffer = wgpu_device.create_buffer(desc)
            wgpu_encoder.copy_texture_to_buffer(source, destination, copy_size)
        let gfx = Arc::new(Gfx {
            device: wgpu_device,
            queue: wgpu_queue,
        });
        //gpu init end
        let token = match CaptureStream::test_access(false) {
            Some(token) => token,
            None => CaptureStream::request_access(false).await.expect("Expected capture access")
        };
        let filter = CapturableContentFilter::NORMAL_WINDOWS;
        let content = CapturableContent::new(filter).await.unwrap();
        let window = content.windows().filter(|window| {
            let app_identifier = window.application().identifier();
            window.title().len() != 0 && app_identifier.to_lowercase().contains("chrome")
        }).next();
        match window {
            Some(window) => {
                println!("capturing window: {}", window.title()); 
                let config = CaptureConfig::with_window(window, CapturePixelFormat::Bgra8888)
                .unwrap()
                .with_wgpu_device(gfx.clone())
                .unwrap();
                let mut stream = CaptureStream::new(token, config, move |stream_event| {
                    match stream_event {
                        Ok(event) => {
                            match event {
                                StreamEvent::Video(frame) => {
                                    println!("Got frame: {}", frame.frame_id());
                                    match frame.get_bitmap() {
                                        Ok(bitmap) => {
                                            match bitmap {
                                                crabgrab::feature::bitmap::FrameBitmap::BgraUnorm8x4(data) => {
                                                    println!("format: BgraUnorm8x4");
                                                    let (width, height) = (data.width as u32, data.height as u32);
                                                    let bgra_data = data.data;
                                                    // 创建一个 ImageBuffer
                                                    let mut img_buffer = ImageBuffer::new(width, height);
                                                    // 将 BgraUnorm8x4 数据转换为 Rgba 格式
                                                    for (x, y, pixel) in img_buffer.enumerate_pixels_mut() {
                                                        let index = (y * width + x) as usize;
                                                        let [b, g, r, a] = bgra_data[index];
                                                        *pixel = Rgba([r, g, b, a]);
                                                    }
                                                    // 将 ImageBuffer 转换为 DynamicImagei
                                                    let dynamic_image = DynamicImage::ImageRgba8(img_buffer);
                                                    capture_send(dynamic_image);
                                                    sleep(Duration::from_millis(100));
                                                },
                                                crabgrab::feature::bitmap::FrameBitmap::ArgbUnormPacked2101010(_) => println!("format: ArgbUnormPacked2101010"),
                                                crabgrab::feature::bitmap::FrameBitmap::RgbaF16x4(_) => println!("format: RgbaF16x4"),
                                                crabgrab::feature::bitmap::FrameBitmap::YCbCr(_) => println!("format: YCbCr"),
                                            }
                                        },
                                        Err(e) => {
                                            println!("Bitmap error: {:?}", e);
                                        }
                                    }
                                    //valid on macos
                                    //MetalVideoFramePlaneTexture::Chroma
                                    //MetalVideoFramePlaneTexture::Luminance
                                    match frame.get_metal_texture(MetalVideoFramePlaneTexture::Luminance){
                                        Ok(_) => {
                                            println!("got metal texture");
                                        },
                                        Err(err) => {
                                            println!("got error of get_mental_texture: error is {}", err);
                                        },
                                    }
                                    //invalid on macos
                                    match frame.get_wgpu_texture(WgpuVideoFramePlaneTexture::Chroma, None){
                                        Ok(_) => println!("get_wgpu_texture: Chroma"),
                                        Err(err) => println!("get_wgpu_texture: Chroma, err is {err}"),
                                    }
                                    //invalid on macos
                                    match frame.get_wgpu_texture(WgpuVideoFramePlaneTexture::Luminance, None){
                                        Ok(_) => println!("get_wgpu_texture: Luminance"),
                                        Err(err) => println!("get_wgpu_texture: Luminance, err is {err}"),
                                    }
                                    //valid on macos
                                    match frame.get_wgpu_texture(WgpuVideoFramePlaneTexture::Rgba, None){
                                        Ok(_texture) => {
                                            println!("get_wgpu_texture: Rgba");
                                            //todo
                                            println!("format is : {:?}", _texture.format());
                                            _texture.width()
                                            // wgpu::Buffer
                                        },
                                        Err(err) => println!("get_wgpu_texture: Rgba, err is {err}"),
                                    }
                                },
                                _ => {}
                            }
                        },
                        Err(error) => {
                            println!("Stream error: {:?}", error);
                        }
                    }
                }).unwrap();
                println!("stream created!"); 
                tokio::task::block_in_place(|| std::thread::sleep(Duration::from_millis(30000)));
                stream.stop().unwrap();
            },
            None => { println!("Failed to find window"); }
        }
    });
    runtime.block_on(future).unwrap();
    runtime.shutdown_timeout(Duration::from_millis(100000));
}
