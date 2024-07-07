#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::{thread::sleep, time::Duration};
mod app;
mod common;
mod event;
mod impls;
#[macro_use]
mod macros;
fn main() {
    env_logger::init();
    app::App::new().run();
    sleep(Duration::from_secs(3600));
}
