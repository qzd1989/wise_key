#[allow(unused_imports)]
use crate::i;
use crabgrab::prelude::*;
use eframe::CreationContext;
use egui::{mutex::RwLock, ColorImage, ImageData};
use egui_wgpu::Renderer;
use image::{ ImageBuffer, Rgba};
#[allow(unused_imports)]
use log::{info, warn};
use pollster::block_on;
use std::{
    sync::{Arc, Mutex, MutexGuard},
    thread::{spawn, JoinHandle},
    time::Duration,
};

#[derive(PartialEq)]
enum State {
    Run,
    Stop,
}
pub struct Capture {
    state: State,
    listen_handle: Option<JoinHandle<()>>,
    pub app: Arc<Mutex<App>>,
}

impl Capture {
    pub fn new(_cc: &CreationContext) -> Self {
        let app = App::new(_cc);
        Self {
            state: State::Stop,
            listen_handle: None,
            app: Arc::new(Mutex::new(app)),
        }
    }
    pub fn app(&self) -> MutexGuard<App> {
        self.app.lock().unwrap()
    }
    pub fn is_stop(&self) -> bool {
        self.state == State::Stop
    }
    pub fn run(&mut self, ctx: egui::Context, frame: &mut eframe::Frame) {
        self.state = State::Run;
        let egui_render_state = frame.wgpu_render_state().unwrap();
        let device = &egui_render_state.device;
        let _adapter = &egui_render_state.adapter;
        let queue = &egui_render_state.queue;
        let renderer = &egui_render_state.renderer;
        let _target_format = egui_render_state.target_format;
        let device_clone = Arc::clone(&device);
        let renderer_clone = Arc::clone(&renderer);
        let queue_clone = Arc::clone(&queue);
        let app_clone = Arc::clone(&self.app);
        self.listen_handle = Some(spawn(move || {
            Self::_listen(device_clone, queue_clone, renderer_clone, app_clone, ctx)
        }));
    }
    fn _listen(
        device: Arc<wgpu::Device>,
        queue: Arc<wgpu::Queue>,
        renderer: Arc<RwLock<Renderer>>,
        app: Arc<Mutex<App>>,
        ctx: egui::Context,
    ) {
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
        let device_clone = Arc::clone(&device);
        let gfx = Arc::new(Gfx {
            device: device_clone,
            queue,
        });
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
                                match frame.get_bitmap() {
                                    Ok(bitmap) => {
                                        match bitmap {
                                            crabgrab::feature::bitmap::FrameBitmap::BgraUnorm8x4(data) => {
                                                let (width, height) = (data.width as u32, data.height as u32);
                                                    let bgra_data = data.data;
                                                    // 创建一个 ImageBuffer
                                                    let mut img_buffer = image::ImageBuffer::new(width, height);
                                                    // 将 BgraUnorm8x4 数据转换为 Rgba 格式
                                                    for (x, y, pixel) in img_buffer.enumerate_pixels_mut() {
                                                        let index = (y * width + x) as usize;
                                                        let [b, g, r, a] = bgra_data[index];
                                                        *pixel = image::Rgba([r, g, b, a]);
                                                    }
                                                    let image_data = image_buffer_to_image_data(img_buffer);
                                                    let mut lock = app.try_lock();
                                                    if let Ok(ref mut mutex) = lock {
                                                        mutex.update_texture_handle(ctx.clone(), &image_data);
                                                    } else {
                                                        println!("try_lock failed");
                                                    }                                                                   
                                                    // std::thread::sleep(Duration::from_millis(1000));
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

pub struct App {
    pub texture_handle: Option<egui::TextureHandle>,
}
impl App {
    pub fn new(_cc: &CreationContext) -> Self {
        Self {
            texture_handle: None,
        }
    }
    pub fn update_texture_handle(&mut self, ctx: egui::Context, data: &ImageData) {
        if let Some(_) = &self.texture_handle {
            self.texture_handle
                .as_mut()
                .unwrap()
                .set(data.clone(), Default::default());
        } else {
            self.texture_handle = Some(ctx.load_texture("haha", data.clone(), Default::default()));
        }
        ctx.request_repaint();
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let _ = frame;
        egui::CentralPanel::default().show(ctx, |ui| {
            let _ = ui.button("hello");
            if let Some(handle) = &self.texture_handle {
                ui.image(handle);
            }
        });
    }
}

fn image_buffer_to_image_data(image_buffer: ImageBuffer<Rgba<u8>, Vec<u8>>) -> ImageData {
    // 获取图像的宽度和高度
    let width = image_buffer.width() as usize;
    let height = image_buffer.height() as usize;
    // 提取像素数据
    let pixels: Vec<u8> = image_buffer.into_raw();
    // 创建 ColorImage
    let color_image = ColorImage::from_rgba_unmultiplied([width, height], &pixels);
    // 包装在 ImageData 中
    ImageData::Color(Arc::new(color_image))
}
