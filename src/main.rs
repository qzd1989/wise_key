mod app;
mod common;
mod event;
mod global;
mod impls;
use app::App;
use std::{thread, time::Duration};

fn main() {
    App::new().run();
    thread::sleep(Duration::from_secs(3600));
}
