use crabgrab::prelude::*;
use eframe::CreationContext;
use egui::{
    mutex::{Mutex, MutexGuard},
    ColorImage, ImageData,
};
use image::DynamicImage;
use pollster::block_on;
use std::{
    sync::Arc,
    thread::{sleep, spawn, JoinHandle},
    time::Duration,
};
use wgpu::Texture;

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
        self.app.lock()
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
        let _renderer = &egui_render_state.renderer;
        let _target_format = egui_render_state.target_format;
        let device_clone = Arc::clone(&device);
        let queue_clone = Arc::clone(&queue);
        let app_clone = Arc::clone(&self.app);
        self.listen_handle = Some(spawn(move || {
            Self::_listen(device_clone, queue_clone, app_clone, ctx)
        }));
    }
    fn _listen(
        device: Arc<wgpu::Device>,
        queue: Arc<wgpu::Queue>,
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
                                match frame.get_bitmap() {
                                    Ok(bitmap) => {
                                        match bitmap {
                                            crabgrab::feature::bitmap::FrameBitmap::BgraUnorm8x4(data) => {
                                                //here you are
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
                                                    // 将 ImageBuffer 转换为 DynamicImagei
                                                    let dynamic_image = image::DynamicImage::ImageRgba8(img_buffer);

                                                    app.lock().image = Some(dynamic_image);
                                                    ctx.request_repaint();
                                                    sleep(Duration::from_millis(50));
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
                                match frame.get_wgpu_texture(WgpuVideoFramePlaneTexture::Rgba, None)
                                {
                                    Ok(texture) => {
                                        app.lock().texture = Some(texture);
                                        println!("texture is : {:?}", app.lock().texture);
                                        ctx.request_repaint();
                                        sleep(Duration::from_millis(70));
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

pub struct App {
    pub image: Option<image::DynamicImage>,
    pub texture: Option<Texture>,
}
impl App {
    pub fn new(_cc: &CreationContext) -> Self {
        Self {
            texture: None,
            image: None,
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let _ = frame;
        egui::CentralPanel::default().show(ctx, |ui| {
            let _ = ui.button("hello");
            // ui.label(format!("{:?}", self.texture));
            if let Some(image) = &self.image {
                let image = dynamic_image_to_egui_image_data(&image);
                let texture = ui.ctx().load_texture("frame", image, Default::default());
                ui.image(&texture);
            }
        });
    }
}

fn dynamic_image_to_egui_image_data(dynamic_image: &DynamicImage) -> ImageData {
    let rgba_image = dynamic_image.to_rgba8();
    let (width, height) = rgba_image.dimensions();
    let pixels = rgba_image.into_raw();

    let color_image =
        ColorImage::from_rgba_unmultiplied([width as usize, height as usize], &pixels);
    ImageData::Color(Arc::new(color_image))
}
