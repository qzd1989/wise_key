#[allow(unused_imports)]
use crate::i;
#[allow(unused_imports)]
use log::{info, warn};

use super::{common::virtual_path, current_point, Button, Event, Key};
use crate::{
    common::{Float, Int, UInt, SIMULATE_STATE_CHANNEL},
    event::SimulateError,
};
use arboard::Clipboard;
use rhai::{Engine, Scope};
use std::time::Duration;

fn drag<T, F, I, G, Q>(from_point: (T, F), to_point: (I, G), duration: Q, button: Button)
where
    T: Into<Float>,
    F: Into<Float>,
    I: Into<Float>,
    G: Into<Float>,
    Q: Into<Float>,
{
    let duration = duration.into() as u128;
    if let Some(points) = virtual_path(
        from_point.0,
        from_point.1,
        to_point.0,
        to_point.1,
        duration as UInt,
    ) {
        points.iter().for_each(move |point| {
            let _ = Event::Drag {
                button,
                x: point.0 as Float,
                y: point.1 as Float,
                elapse: 0,
                duration: point.2,
            }
            .simulate();
        });
    }
}
fn click<T, F>(point: (T, F), button: Button)
where
    T: Into<Float>,
    F: Into<Float>,
{
    let (x, y) = (point.0.into(), point.1.into());
    let _ = Event::ButtonPress {
        button,
        x,
        y,
        elapse: 0,
        duration: 0,
    }
    .simulate();
    let _ = Event::ButtonRelease {
        button,
        x,
        y,
        elapse: 0,
        duration: 10,
    }
    .simulate();
}

fn wheel<T>(y: T)
where
    T: Into<Float>,
{
    let y = y.into() as Int;
    let _ = Event::Wheel {
        x: 0,
        y,
        elapse: 0,
        duration: 0,
    }
    .simulate();
}

pub fn drag_left_instant<T, F>(x: T, y: F)
where
    T: Into<Float>,
    F: Into<Float>,
{
    let (x, y) = (x.into(), y.into());
    let _ = Event::Drag {
        button: Button::Left,
        x,
        y,
        elapse: 0,
        duration: 0,
    }
    .simulate();
}

pub fn drag_right_instant<T, F>(x: T, y: F)
where
    T: Into<Float>,
    F: Into<Float>,
{
    let (x, y) = (x.into(), y.into());
    let _ = Event::Drag {
        button: Button::Right,
        x,
        y,
        elapse: 0,
        duration: 0,
    }
    .simulate();
}

pub fn drag_left_relative<T, F, I, G, Q>(from_point: (T, F), offset: (I, G), duration: Q)
where
    T: Into<Float>,
    F: Into<Float>,
    I: Into<Float>,
    G: Into<Float>,
    Q: Into<Float>,
{
    let (from_point, offset, duration) = (
        (from_point.0.into(), from_point.1.into()),
        (offset.0.into(), offset.1.into()),
        duration.into(),
    );
    let to_point = (from_point.0 + offset.0, from_point.1 + offset.1);
    drag(from_point, to_point, duration, Button::Left);
}
pub fn drag_right_relative<T, F, I, G, Q>(from_point: (T, F), offset: (I, G), duration: Q)
where
    T: Into<Float>,
    F: Into<Float>,
    I: Into<Float>,
    G: Into<Float>,
    Q: Into<Float>,
{
    let (from_point, offset, duration) = (
        (from_point.0.into(), from_point.1.into()),
        (offset.0.into(), offset.1.into()),
        duration.into(),
    );
    let to_point = (from_point.0 + offset.0, from_point.1 + offset.1);
    drag(from_point, to_point, duration, Button::Right);
}
pub fn click_left<T, F>(x: T, y: F)
where
    T: Into<Float>,
    F: Into<Float>,
{
    click((x, y), Button::Left);
}
pub fn click_right<T, F>(x: T, y: F)
where
    T: Into<Float>,
    F: Into<Float>,
{
    click((x, y), Button::Right);
}
pub fn button_left_click() {
    click(current_point(), Button::Left);
}
pub fn button_right_click() {
    click(current_point(), Button::Right);
}
pub fn button_left_press<T, F>(x: T, y: F)
where
    T: Into<Float>,
    F: Into<Float>,
{
    let (x, y) = (x.into(), y.into());
    let _ = Event::ButtonPress {
        button: Button::Left,
        x,
        y,
        elapse: 0,
        duration: 0,
    }
    .simulate();
}
pub fn button_left_release<T, F>(x: T, y: F)
where
    T: Into<Float>,
    F: Into<Float>,
{
    let (x, y) = (x.into(), y.into());
    let _ = Event::ButtonRelease {
        button: Button::Left,
        x,
        y,
        elapse: 0,
        duration: 0,
    }
    .simulate();
}

pub fn button_right_press<T, F>(x: T, y: F)
where
    T: Into<Float>,
    F: Into<Float>,
{
    let (x, y) = (x.into(), y.into());
    let _ = Event::ButtonPress {
        button: Button::Right,
        x,
        y,
        elapse: 0,
        duration: 0,
    }
    .simulate();
}

pub fn button_right_release<T, F>(x: T, y: F)
where
    T: Into<Float>,
    F: Into<Float>,
{
    let (x, y) = (x.into(), y.into());
    let _ = Event::ButtonRelease {
        button: Button::Right,
        x,
        y,
        elapse: 0,
        duration: 0,
    }
    .simulate();
}

pub fn mouse_move<T, F>(x: T, y: F)
where
    T: Into<Float>,
    F: Into<Float>,
{
    let x = x.into();
    let y = y.into();
    let _ = Event::MouseMove {
        x: x as Float,
        y: y as Float,
        elapse: 0,
        duration: 0,
    }
    .simulate();
}

pub fn mouse_move_relative<T, F, I, Q>(point: (T, F), offset: (I, Q)) -> (Float, Float)
where
    T: Into<Float>,
    F: Into<Float>,
    I: Into<Float>,
    Q: Into<Float>,
{
    let (point, offset) = (
        (point.0.into(), point.1.into()),
        (offset.0.into(), offset.1.into()),
    );
    let x = point.0 + offset.0;
    let y = point.1 + offset.1;
    mouse_move(x, y);
    (x, y)
}
pub fn wheel_up<T>(delta_y: T)
where
    T: Into<Float>,
{
    let delta_y = delta_y.into() as Int;
    let delta_y = 0 - delta_y.abs();
    wheel(delta_y);
}
pub fn wheel_down<T>(delta_y: T)
where
    T: Into<Float>,
{
    let delta_y = delta_y.into() as Int;
    wheel(delta_y.abs());
}
pub fn key_press(key: Key) {
    let _ = Event::KeyPress {
        key,
        elapse: 0,
        duration: 0,
    }
    .simulate();
}
pub fn key_release(key: Key) {
    let _ = Event::KeyRelease {
        key,
        elapse: 0,
        duration: 0,
    }
    .simulate();
}
pub fn key_click(key: Key) {
    key_press(key);
    delay(10);
    key_release(key);
}
pub fn delay<T>(duration: T)
where
    T: Into<Float>,
{
    let duration = duration.into() as u64;
    std::thread::sleep(Duration::from_millis(duration));
}

#[cfg(target_os = "macos")]
pub fn select_all() {
    key_press(Key::MetaLeft);
    delay(10);
    key_press(Key::KeyA);
    delay(5);
    key_release(Key::KeyA);
    delay(5);
    key_release(Key::MetaLeft);
}

#[cfg(target_os = "windows")]
pub fn select_all() {
    let mut clipboard = Clipboard::new().unwrap();
    clipboard.set_text(text).unwrap();
    key_press(Key::ControlLeft);
    delay(10);
    key_press(Key::KeyA);
    delay(5);
    key_release(Key::KeyA);
    delay(5);
    key_release(Key::ControlLeft);
}

#[cfg(target_os = "macos")]
pub fn paste_text(text: &str) {
    let mut clipboard = Clipboard::new().unwrap();
    clipboard.set_text(text).unwrap();
    key_press(Key::MetaLeft);
    delay(10);
    key_press(Key::KeyV);
    delay(5);
    key_release(Key::KeyV);
    delay(5);
    key_release(Key::MetaLeft);
}

#[cfg(target_os = "windows")]
pub fn paste_text(text: &str) {
    let mut clipboard = Clipboard::new().unwrap();
    clipboard.set_text(text).unwrap();
    key_press(Key::ControlLeft);
    delay(10);
    key_press(Key::KeyV);
    delay(5);
    key_release(Key::KeyV);
    delay(5);
    key_release(Key::ControlLeft);
}

pub fn run(content: String) -> Result<(), SimulateError> {
    let mut engine = Engine::new();
    let mut scope = Scope::new();

    scope.push("Alt", Key::Alt);
    scope.push("AltGr", Key::AltGr);
    scope.push("Backspace", Key::Backspace);
    scope.push("CapsLock", Key::CapsLock);
    scope.push("ControlLeft", Key::ControlLeft);
    scope.push("ControlRight", Key::ControlRight);
    scope.push("Delete", Key::Delete);
    scope.push("DownArrow", Key::DownArrow);
    scope.push("End", Key::End);
    scope.push("Escape", Key::Escape);
    scope.push("f1", Key::F1);
    scope.push("f10", Key::F10);
    scope.push("f11", Key::F11);
    scope.push("f12", Key::F12);
    scope.push("f2", Key::F2);
    scope.push("f3", Key::F3);
    scope.push("f4", Key::F4);
    scope.push("f5", Key::F5);
    scope.push("f6", Key::F6);
    scope.push("f7", Key::F7);
    scope.push("f8", Key::F8);
    scope.push("f9", Key::F9);
    scope.push("Home", Key::Home);
    scope.push("LeftArrow", Key::LeftArrow);
    scope.push("MetaLeft", Key::MetaLeft);
    scope.push("MetaRight", Key::MetaRight);
    scope.push("PageDown", Key::PageDown);
    scope.push("PageUp", Key::PageUp);
    scope.push("Return", Key::Return);
    scope.push("RightArrow", Key::RightArrow);
    scope.push("ShiftLeft", Key::ShiftLeft);
    scope.push("ShiftRight", Key::ShiftRight);
    scope.push("Space", Key::Space);
    scope.push("Tab", Key::Tab);
    scope.push("UpArrow", Key::UpArrow);
    scope.push("PrintScreen", Key::PrintScreen);
    scope.push("ScrollLock", Key::ScrollLock);
    scope.push("Pause", Key::Pause);
    scope.push("NumLock", Key::NumLock);
    scope.push("BackQuote", Key::BackQuote);
    scope.push("n1", Key::Num1);
    scope.push("n2", Key::Num2);
    scope.push("n3", Key::Num3);
    scope.push("n4", Key::Num4);
    scope.push("n5", Key::Num5);
    scope.push("n6", Key::Num6);
    scope.push("n7", Key::Num7);
    scope.push("n8", Key::Num8);
    scope.push("n9", Key::Num9);
    scope.push("n0", Key::Num0);
    scope.push("Minus", Key::Minus);
    scope.push("Equal", Key::Equal);
    scope.push("q", Key::KeyQ);
    scope.push("w", Key::KeyW);
    scope.push("e", Key::KeyE);
    scope.push("r", Key::KeyR);
    scope.push("t", Key::KeyT);
    scope.push("y", Key::KeyY);
    scope.push("u", Key::KeyU);
    scope.push("i", Key::KeyI);
    scope.push("o", Key::KeyO);
    scope.push("p", Key::KeyP);
    scope.push("LeftBracket", Key::LeftBracket);
    scope.push("RightBracket", Key::RightBracket);
    scope.push("a", Key::KeyA);
    scope.push("s", Key::KeyS);
    scope.push("d", Key::KeyD);
    scope.push("f", Key::KeyF);
    scope.push("g", Key::KeyG);
    scope.push("h", Key::KeyH);
    scope.push("j", Key::KeyJ);
    scope.push("k", Key::KeyK);
    scope.push("l", Key::KeyL);
    scope.push("SemiColon", Key::SemiColon);
    scope.push("Quote", Key::Quote);
    scope.push("BackSlash", Key::BackSlash);
    scope.push("IntlBackslash", Key::IntlBackslash);
    scope.push("z", Key::KeyZ);
    scope.push("x", Key::KeyX);
    scope.push("c", Key::KeyC);
    scope.push("v", Key::KeyV);
    scope.push("b", Key::KeyB);
    scope.push("n", Key::KeyN);
    scope.push("m", Key::KeyM);
    scope.push("Comma", Key::Comma);
    scope.push("Dot", Key::Dot);
    scope.push("Slash", Key::Slash);
    scope.push("Insert", Key::Insert);
    scope.push("KpReturn", Key::KpReturn);
    scope.push("KpMinus", Key::KpMinus);
    scope.push("KpPlus", Key::KpPlus);
    scope.push("KpMultiply", Key::KpMultiply);
    scope.push("KpDivide", Key::KpDivide);
    scope.push("Kp0", Key::Kp0);
    scope.push("Kp1", Key::Kp1);
    scope.push("Kp2", Key::Kp2);
    scope.push("Kp3", Key::Kp3);
    scope.push("Kp4", Key::Kp4);
    scope.push("Kp5", Key::Kp5);
    scope.push("Kp6", Key::Kp6);
    scope.push("Kp7", Key::Kp7);
    scope.push("Kp8", Key::Kp8);
    scope.push("Kp9", Key::Kp9);
    scope.push("KpDelete", Key::KpDelete);
    scope.push("Function", Key::Function);
    engine
        .register_fn("drag_left_instant", drag_left_instant::<Int, Int>)
        .register_fn("drag_left_instant", drag_left_instant::<Float, Float>)
        .register_fn("drag_left_instant", drag_left_instant::<Int, Float>)
        .register_fn("drag_left_instant", drag_left_instant::<Float, Int>);
    engine
        .register_fn("drag_right_instant", drag_right_instant::<Int, Int>)
        .register_fn("drag_right_instant", drag_right_instant::<Float, Float>)
        .register_fn("drag_right_instant", drag_right_instant::<Int, Float>)
        .register_fn("drag_right_instant", drag_right_instant::<Float, Int>);
    engine
        .register_fn(
            "drag_left_relative",
            drag_left_relative::<Int, Int, Int, Int, Int>,
        )
        .register_fn(
            "drag_left_relative",
            drag_left_relative::<Int, Int, Int, Int, Float>,
        )
        .register_fn(
            "drag_left_relative",
            drag_left_relative::<Int, Int, Int, Float, Int>,
        )
        .register_fn(
            "drag_left_relative",
            drag_left_relative::<Int, Int, Int, Float, Float>,
        )
        .register_fn(
            "drag_left_relative",
            drag_left_relative::<Int, Int, Float, Int, Int>,
        )
        .register_fn(
            "drag_left_relative",
            drag_left_relative::<Int, Int, Float, Int, Float>,
        )
        .register_fn(
            "drag_left_relative",
            drag_left_relative::<Int, Int, Float, Float, Int>,
        )
        .register_fn(
            "drag_left_relative",
            drag_left_relative::<Int, Int, Float, Float, Float>,
        )
        .register_fn(
            "drag_left_relative",
            drag_left_relative::<Int, Float, Int, Int, Int>,
        )
        .register_fn(
            "drag_left_relative",
            drag_left_relative::<Int, Float, Int, Int, Float>,
        )
        .register_fn(
            "drag_left_relative",
            drag_left_relative::<Int, Float, Int, Float, Int>,
        )
        .register_fn(
            "drag_left_relative",
            drag_left_relative::<Int, Float, Int, Float, Float>,
        )
        .register_fn(
            "drag_left_relative",
            drag_left_relative::<Int, Float, Float, Int, Int>,
        )
        .register_fn(
            "drag_left_relative",
            drag_left_relative::<Int, Float, Float, Int, Float>,
        )
        .register_fn(
            "drag_left_relative",
            drag_left_relative::<Int, Float, Float, Float, Int>,
        )
        .register_fn(
            "drag_left_relative",
            drag_left_relative::<Int, Float, Float, Float, Float>,
        )
        .register_fn(
            "drag_left_relative",
            drag_left_relative::<Float, Int, Int, Int, Int>,
        )
        .register_fn(
            "drag_left_relative",
            drag_left_relative::<Float, Int, Int, Int, Float>,
        )
        .register_fn(
            "drag_left_relative",
            drag_left_relative::<Float, Int, Int, Float, Int>,
        )
        .register_fn(
            "drag_left_relative",
            drag_left_relative::<Float, Int, Int, Float, Float>,
        )
        .register_fn(
            "drag_left_relative",
            drag_left_relative::<Float, Int, Float, Int, Int>,
        )
        .register_fn(
            "drag_left_relative",
            drag_left_relative::<Float, Int, Float, Int, Float>,
        )
        .register_fn(
            "drag_left_relative",
            drag_left_relative::<Float, Int, Float, Float, Int>,
        )
        .register_fn(
            "drag_left_relative",
            drag_left_relative::<Float, Int, Float, Float, Float>,
        )
        .register_fn(
            "drag_left_relative",
            drag_left_relative::<Float, Float, Int, Int, Int>,
        )
        .register_fn(
            "drag_left_relative",
            drag_left_relative::<Float, Float, Int, Int, Float>,
        )
        .register_fn(
            "drag_left_relative",
            drag_left_relative::<Float, Float, Int, Float, Int>,
        )
        .register_fn(
            "drag_left_relative",
            drag_left_relative::<Float, Float, Int, Float, Float>,
        )
        .register_fn(
            "drag_left_relative",
            drag_left_relative::<Float, Float, Float, Int, Int>,
        )
        .register_fn(
            "drag_left_relative",
            drag_left_relative::<Float, Float, Float, Int, Float>,
        )
        .register_fn(
            "drag_left_relative",
            drag_left_relative::<Float, Float, Float, Float, Int>,
        )
        .register_fn(
            "drag_left_relative",
            drag_left_relative::<Float, Float, Float, Float, Float>,
        );
    engine
        .register_fn(
            "drag_right_relative",
            drag_right_relative::<Int, Int, Int, Int, Int>,
        )
        .register_fn(
            "drag_right_relative",
            drag_right_relative::<Int, Int, Int, Int, Float>,
        )
        .register_fn(
            "drag_right_relative",
            drag_right_relative::<Int, Int, Int, Float, Int>,
        )
        .register_fn(
            "drag_right_relative",
            drag_right_relative::<Int, Int, Int, Float, Float>,
        )
        .register_fn(
            "drag_right_relative",
            drag_right_relative::<Int, Int, Float, Int, Int>,
        )
        .register_fn(
            "drag_right_relative",
            drag_right_relative::<Int, Int, Float, Int, Float>,
        )
        .register_fn(
            "drag_right_relative",
            drag_right_relative::<Int, Int, Float, Float, Int>,
        )
        .register_fn(
            "drag_right_relative",
            drag_right_relative::<Int, Int, Float, Float, Float>,
        )
        .register_fn(
            "drag_right_relative",
            drag_right_relative::<Int, Float, Int, Int, Int>,
        )
        .register_fn(
            "drag_right_relative",
            drag_right_relative::<Int, Float, Int, Int, Float>,
        )
        .register_fn(
            "drag_right_relative",
            drag_right_relative::<Int, Float, Int, Float, Int>,
        )
        .register_fn(
            "drag_right_relative",
            drag_right_relative::<Int, Float, Int, Float, Float>,
        )
        .register_fn(
            "drag_right_relative",
            drag_right_relative::<Int, Float, Float, Int, Int>,
        )
        .register_fn(
            "drag_right_relative",
            drag_right_relative::<Int, Float, Float, Int, Float>,
        )
        .register_fn(
            "drag_right_relative",
            drag_right_relative::<Int, Float, Float, Float, Int>,
        )
        .register_fn(
            "drag_right_relative",
            drag_right_relative::<Int, Float, Float, Float, Float>,
        )
        .register_fn(
            "drag_right_relative",
            drag_right_relative::<Float, Int, Int, Int, Int>,
        )
        .register_fn(
            "drag_right_relative",
            drag_right_relative::<Float, Int, Int, Int, Float>,
        )
        .register_fn(
            "drag_right_relative",
            drag_right_relative::<Float, Int, Int, Float, Int>,
        )
        .register_fn(
            "drag_right_relative",
            drag_right_relative::<Float, Int, Int, Float, Float>,
        )
        .register_fn(
            "drag_right_relative",
            drag_right_relative::<Float, Int, Float, Int, Int>,
        )
        .register_fn(
            "drag_right_relative",
            drag_right_relative::<Float, Int, Float, Int, Float>,
        )
        .register_fn(
            "drag_right_relative",
            drag_right_relative::<Float, Int, Float, Float, Int>,
        )
        .register_fn(
            "drag_right_relative",
            drag_right_relative::<Float, Int, Float, Float, Float>,
        )
        .register_fn(
            "drag_right_relative",
            drag_right_relative::<Float, Float, Int, Int, Int>,
        )
        .register_fn(
            "drag_right_relative",
            drag_right_relative::<Float, Float, Int, Int, Float>,
        )
        .register_fn(
            "drag_right_relative",
            drag_right_relative::<Float, Float, Int, Float, Int>,
        )
        .register_fn(
            "drag_right_relative",
            drag_right_relative::<Float, Float, Int, Float, Float>,
        )
        .register_fn(
            "drag_right_relative",
            drag_right_relative::<Float, Float, Float, Int, Int>,
        )
        .register_fn(
            "drag_right_relative",
            drag_right_relative::<Float, Float, Float, Int, Float>,
        )
        .register_fn(
            "drag_right_relative",
            drag_right_relative::<Float, Float, Float, Float, Int>,
        )
        .register_fn(
            "drag_right_relative",
            drag_right_relative::<Float, Float, Float, Float, Float>,
        );
    engine
        .register_fn("click_left", click_left::<Int, Int>)
        .register_fn("click_left", click_left::<Int, Float>)
        .register_fn("click_left", click_left::<Float, Int>)
        .register_fn("click_left", click_left::<Float, Float>);
    engine
        .register_fn("click_right", click_right::<Int, Int>)
        .register_fn("click_right", click_right::<Int, Float>)
        .register_fn("click_right", click_right::<Float, Int>)
        .register_fn("click_right", click_right::<Float, Float>);
    engine.register_fn("button_left_click", button_left_click);
    engine.register_fn("button_right_click", button_right_click);
    engine
        .register_fn("mouse_move", mouse_move::<Int, Int>)
        .register_fn("mouse_move", mouse_move::<Float, Float>)
        .register_fn("mouse_move", mouse_move::<Int, Float>)
        .register_fn("mouse_move", mouse_move::<Float, Int>);
    engine
        .register_fn(
            "mouse_move_relative",
            mouse_move_relative::<Int, Int, Int, Int>,
        )
        .register_fn(
            "mouse_move_relative",
            mouse_move_relative::<Int, Int, Int, Float>,
        )
        .register_fn(
            "mouse_move_relative",
            mouse_move_relative::<Int, Int, Float, Int>,
        )
        .register_fn(
            "mouse_move_relative",
            mouse_move_relative::<Int, Int, Float, Float>,
        )
        .register_fn(
            "mouse_move_relative",
            mouse_move_relative::<Int, Float, Int, Int>,
        )
        .register_fn(
            "mouse_move_relative",
            mouse_move_relative::<Int, Float, Int, Float>,
        )
        .register_fn(
            "mouse_move_relative",
            mouse_move_relative::<Int, Float, Float, Int>,
        )
        .register_fn(
            "mouse_move_relative",
            mouse_move_relative::<Int, Float, Float, Float>,
        )
        .register_fn(
            "mouse_move_relative",
            mouse_move_relative::<Float, Int, Int, Int>,
        )
        .register_fn(
            "mouse_move_relative",
            mouse_move_relative::<Float, Int, Int, Float>,
        )
        .register_fn(
            "mouse_move_relative",
            mouse_move_relative::<Float, Int, Float, Int>,
        )
        .register_fn(
            "mouse_move_relative",
            mouse_move_relative::<Float, Int, Float, Float>,
        )
        .register_fn(
            "mouse_move_relative",
            mouse_move_relative::<Float, Float, Int, Int>,
        )
        .register_fn(
            "mouse_move_relative",
            mouse_move_relative::<Float, Float, Int, Float>,
        )
        .register_fn(
            "mouse_move_relative",
            mouse_move_relative::<Float, Float, Float, Int>,
        )
        .register_fn(
            "mouse_move_relative",
            mouse_move_relative::<Float, Float, Float, Float>,
        );
    engine
        .register_fn("wheel_up", wheel_up::<Int>)
        .register_fn("wheel_up", wheel_up::<Float>);
    engine
        .register_fn("wheel_down", wheel_down::<Int>)
        .register_fn("wheel_down", wheel_down::<Float>);
    engine
        .register_fn("delay", delay::<Int>)
        .register_fn("delay", delay::<Float>);
    engine.register_fn("key_press", key_press);
    engine.register_fn("key_release", key_release);
    engine.register_fn("key_click", key_click);
    engine.register_fn("paste_text", paste_text);
    engine.register_fn("select_all", select_all);
    engine
        .register_fn("button_left_press", button_left_press::<Int, Int>)
        .register_fn("button_left_press", button_left_press::<Int, Float>)
        .register_fn("button_left_press", button_left_press::<Float, Int>)
        .register_fn("button_left_press", button_left_press::<Float, Float>);
    engine
        .register_fn("button_left_release", button_left_release::<Int, Int>)
        .register_fn("button_left_release", button_left_release::<Int, Float>)
        .register_fn("button_left_release", button_left_release::<Float, Int>)
        .register_fn("button_left_release", button_left_release::<Float, Float>);
    engine
        .register_fn("button_right_press", button_right_press::<Int, Int>)
        .register_fn("button_right_press", button_right_press::<Int, Float>)
        .register_fn("button_right_press", button_right_press::<Float, Int>)
        .register_fn("button_right_press", button_right_press::<Float, Float>);
    engine
        .register_fn("button_right_release", button_right_release::<Int, Int>)
        .register_fn("button_right_release", button_right_release::<Int, Float>)
        .register_fn("button_right_release", button_right_release::<Float, Int>)
        .register_fn("button_right_release", button_right_release::<Float, Float>);

    engine.on_progress(move |_opt| loop {
        match SIMULATE_STATE_CHANNEL.1.try_recv() {
            Ok(result) if result => {
                info!("get true form SIMULATE_STATE_CHANNEL, simulating should be paused");
                return Some("stop".into());
            }
            Ok(_) => return None,
            Err(_) => return None,
        }
    });
    if let Err(err) = engine.run_with_scope(&mut scope, content.as_str()) {
        return Err(SimulateError::Rhai(err));
    }
    Ok(())
}
