#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use app::App;
use eframe::egui;
mod app;
mod capture;
mod common;
mod event;
mod impls;
#[macro_use]
mod macros;
fn main() {
    env_logger::init();
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([800.0, 800.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Image Viewer",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::new(App::new(cc)))
        }),
    )
    .unwrap();
}
