use crabgrab::prelude::*;
use eframe::CreationContext;
use pollster::block_on;
use std::{
    sync::Arc,
    thread::{spawn, JoinHandle},
    time::Duration,
};

#[derive(PartialEq)]
enum State {
    Run,
    Stop,
}
pub struct Server {
    state: State,
    listen_handle: Option<JoinHandle<()>>,
}

impl Server {
    pub fn new(_cc: &CreationContext) -> Self {
        Self {
            state: State::Stop,
            listen_handle: None,
        }
    }

    pub fn is_stop(&self) -> bool {
        self.state == State::Stop
    }

    pub fn run(&mut self, _ctx: egui::Context, frame: &mut eframe::Frame) {
        self.state = State::Run;
        let egui_render_state = frame.wgpu_render_state().unwrap();
        let device = &egui_render_state.device;
        let _adapter = &egui_render_state.adapter;
        let queue = &egui_render_state.queue;
        let _renderer = &egui_render_state.renderer;
        let _target_format = egui_render_state.target_format;
        let device_clone = Arc::clone(&device);
        let queue_clone = Arc::clone(&queue);
        self.listen_handle = Some(spawn(move || Self::_listen(device_clone, queue_clone)));
    }

    fn _listen(device: Arc<wgpu::Device>, queue: Arc<wgpu::Queue>) {
        let token = block_on(async {
            match CaptureStream::test_access(false) {
                Some(token) => token,
                None => CaptureStream::request_access(false)
                    .await
                    .expect("Expected capture access"),
            }
        });
        let filter = CapturableContentFilter::NORMAL_WINDOWS;
        let content = block_on(async { CapturableContent::new(filter).await.unwrap() });
        let window = content
            .windows()
            .filter(|window| {
                let app_identifier = window.application().identifier();
                window.title().len() != 0 && app_identifier.to_lowercase().contains("chrome")
            })
            .next();
        let gfx = Arc::new(Gfx { device, queue });
        match window {
            Some(window) => {
                println!("capturing window: {}", window.title());
                let config = CaptureConfig::with_window(window, CapturePixelFormat::Bgra8888)
                    .unwrap()
                    .with_wgpu_device(gfx.clone())
                    .unwrap();
                let mut stream =
                    CaptureStream::new(token, config, move |stream_event| match stream_event {
                        Ok(event) => match event {
                            StreamEvent::Video(frame) => {
                                match frame.get_wgpu_texture(WgpuVideoFramePlaneTexture::Rgba, None)
                                {
                                    Ok(_texture) => {
                                        println!("get_wgpu_texture: Rgba");
                                    }
                                    Err(err) => {
                                        println!("get_wgpu_texture: Rgba, err is {err}")
                                    }
                                }
                            }
                            _ => {}
                        },
                        Err(error) => {
                            println!("Stream error: {:?}", error);
                        }
                    })
                    .unwrap();
                println!("stream created!");
                pollster::block_on(async { std::thread::sleep(Duration::from_millis(3600000)) });
                stream.stop().unwrap();
            }
            None => {
                println!("Failed to find window");
            }
        }
    }
}

#[allow(unused)]
struct Gfx {
    device: Arc<wgpu::Device>,
    queue: Arc<wgpu::Queue>,
}

impl AsRef<wgpu::Device> for Gfx {
    fn as_ref(&self) -> &wgpu::Device {
        &self.device
    }
}

pub struct Client {}
impl Client {
    pub fn new(_cc: &CreationContext) -> Self {
        Self {}
    }
}

impl eframe::App for Client {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let _ = frame;
        egui::CentralPanel::default().show(ctx, |ui| {
            let _ = ui.button("hello");
        });
    }
}
