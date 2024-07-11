use std::time::Instant;
use xcap::Monitor;
use xcap::Window;

fn minitors() {
    let monitors = Monitor::all().unwrap();
    for monitor in monitors {
        println!("monitor: {:?}", monitor);
    }
}

fn windows() {
    let windows = Window::all().unwrap();
    for window in windows {
        if window.is_minimized() {
            continue;
        }
        println!(
            "window: {:?}:{:?} {:?} {:?}",
            window.id(),
            window.title(),
            (window.x(), window.y(), window.width(), window.height()),
            (window.is_minimized(), window.is_maximized())
        );
    }
}

fn main() {}
