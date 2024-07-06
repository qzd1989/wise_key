use crate::event::generate_vritual_path;
use crate::event::Coordinate;
use crate::event::Event;
use arboard::Clipboard;
use rdev::get_current_mouse_location;
use rdev::Button;
use rdev::Key;
use std::time::Duration;
fn drag<T, F, I, G, Q>(from_point: (T, F), to_point: (I, G), duration: Q, button: Button)
where
    T: Into<f64>,
    F: Into<f64>,
    I: Into<f64>,
    G: Into<f64>,
    Q: Into<f64>,
{
    let duration = duration.into() as u128;
    if let Some(vritual_points) =
        generate_vritual_path(from_point.0, from_point.1, to_point.0, to_point.1, duration)
    {
        vritual_points.iter().for_each(move |point| {
            Event::Drag {
                button,
                x: point.0 as f64,
                y: point.1 as f64,
                elapse: 0,
                duration: point.2,
            }
            .simulate();
        });
    }
}
fn click<T, F>(point: (T, F), button: Button)
where
    T: Into<f64>,
    F: Into<f64>,
{
    let (x, y) = (point.0.into(), point.1.into());
    Event::ButtonPress {
        button,
        x,
        y,
        coordinate: Coordinate::Abs,
        elapse: 0,
        duration: 0,
    }
    .simulate();
    Event::ButtonRelease {
        button,
        x,
        y,
        coordinate: Coordinate::Abs,
        elapse: 0,
        duration: 10,
    }
    .simulate();
}

fn wheel<T>(delta_y: T)
where
    T: Into<f64>,
{
    let delta_y = delta_y.into() as i64;
    Event::Wheel {
        delta_x: 0,
        delta_y,
        elapse: 0,
        duration: 0,
    }
    .simulate();
}

pub fn drag_left_instant<T, F>(x: T, y: F)
where
    T: Into<f64>,
    F: Into<f64>,
{
    let (x, y) = (x.into(), y.into());
    Event::Drag {
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
    T: Into<f64>,
    F: Into<f64>,
{
    let (x, y) = (x.into(), y.into());
    Event::Drag {
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
    T: Into<f64>,
    F: Into<f64>,
    I: Into<f64>,
    G: Into<f64>,
    Q: Into<f64>,
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
    T: Into<f64>,
    F: Into<f64>,
    I: Into<f64>,
    G: Into<f64>,
    Q: Into<f64>,
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
    T: Into<f64>,
    F: Into<f64>,
{
    click((x, y), Button::Left);
}
pub fn click_right<T, F>(x: T, y: F)
where
    T: Into<f64>,
    F: Into<f64>,
{
    click((x, y), Button::Right);
}
pub fn button_left_click() {
    if let Some(point) = get_current_mouse_location() {
        click((point.x, point.y), Button::Left);
    }
}
pub fn button_right_click() {
    if let Some(point) = get_current_mouse_location() {
        click((point.x, point.y), Button::Right);
    }
}
pub fn button_left_press<T, F>(x: T, y: F)
where
    T: Into<f64>,
    F: Into<f64>,
{
    let (x, y) = (x.into(), y.into());
    Event::ButtonPress {
        button: Button::Left,
        x,
        y,
        coordinate: Coordinate::Abs,
        elapse: 0,
        duration: 0,
    }
    .simulate();
}
pub fn button_left_release<T, F>(x: T, y: F)
where
    T: Into<f64>,
    F: Into<f64>,
{
    let (x, y) = (x.into(), y.into());
    Event::ButtonRelease {
        button: Button::Left,
        x,
        y,
        coordinate: Coordinate::Abs,
        elapse: 0,
        duration: 0,
    }
    .simulate();
}

pub fn button_right_press<T, F>(x: T, y: F)
where
    T: Into<f64>,
    F: Into<f64>,
{
    let (x, y) = (x.into(), y.into());
    Event::ButtonPress {
        button: Button::Right,
        x,
        y,
        coordinate: Coordinate::Abs,
        elapse: 0,
        duration: 0,
    }
    .simulate();
}

pub fn button_right_release<T, F>(x: T, y: F)
where
    T: Into<f64>,
    F: Into<f64>,
{
    let (x, y) = (x.into(), y.into());
    Event::ButtonRelease {
        button: Button::Right,
        x,
        y,
        coordinate: Coordinate::Abs,
        elapse: 0,
        duration: 0,
    }
    .simulate();
}

pub fn mouse_move<T, F>(x: T, y: F)
where
    T: Into<f64>,
    F: Into<f64>,
{
    let x = x.into();
    let y = y.into();
    Event::MouseMove {
        x: x as f64,
        y: y as f64,
        elapse: 0,
        duration: 0,
    }
    .simulate();
}

pub fn mouse_move_relative<T, F, I, Q>(point: (T, F), offset: (I, Q)) -> (f64, f64)
where
    T: Into<f64>,
    F: Into<f64>,
    I: Into<f64>,
    Q: Into<f64>,
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
    T: Into<f64>,
{
    let delta_y = delta_y.into() as i32;
    let delta_y = 0 - delta_y.abs();
    wheel(delta_y);
}
pub fn wheel_down<T>(delta_y: T)
where
    T: Into<f64>,
{
    let delta_y = delta_y.into() as i32;
    wheel(delta_y.abs());
}
pub fn key_press(key: Key) {
    Event::KeyPress {
        key,
        elapse: 0,
        duration: 0,
    }
    .simulate();
}
pub fn key_release(key: Key) {
    Event::KeyRelease {
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
    T: Into<f64>,
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

pub trait KeyConvert {
    fn as_str(&self) -> &str;
}
impl KeyConvert for Key {
    fn as_str(&self) -> &str {
        match self {
            Key::Alt => "Alt",
            Key::AltGr => "AltGr",
            Key::Backspace => "Backspace",
            Key::CapsLock => "CapsLock",
            Key::ControlLeft => "ControlLeft",
            Key::ControlRight => "ControlRight",
            Key::Delete => "Delete",
            Key::DownArrow => "DownArrow",
            Key::End => "End",
            Key::Escape => "Escape",
            Key::F1 => "f1",
            Key::F10 => "f10",
            Key::F11 => "f11",
            Key::F12 => "f12",
            Key::F2 => "f2",
            Key::F3 => "f3",
            Key::F4 => "f4",
            Key::F5 => "f5",
            Key::F6 => "f6",
            Key::F7 => "f7",
            Key::F8 => "f8",
            Key::F9 => "f9",
            Key::Home => "Home",
            Key::LeftArrow => "LeftArrow",
            Key::MetaLeft => "MetaLeft",
            Key::MetaRight => "MetaRight",
            Key::PageDown => "PageDown",
            Key::PageUp => "PageUp",
            Key::Return => "Return",
            Key::RightArrow => "RightArrow",
            Key::ShiftLeft => "ShiftLeft",
            Key::ShiftRight => "ShiftRight",
            Key::Space => "Space",
            Key::Tab => "Tab",
            Key::UpArrow => "UpArrow",
            Key::PrintScreen => "PrintScreen",
            Key::ScrollLock => "ScrollLock",
            Key::Pause => "Pause",
            Key::NumLock => "NumLock",
            Key::BackQuote => "BackQuote",
            Key::Num1 => "n1",
            Key::Num2 => "n",
            Key::Num3 => "n3",
            Key::Num4 => "n4",
            Key::Num5 => "n5",
            Key::Num6 => "n6",
            Key::Num7 => "n7",
            Key::Num8 => "n8",
            Key::Num9 => "n9",
            Key::Num0 => "n0",
            Key::Minus => "Minus",
            Key::Equal => "Equal",
            Key::KeyQ => "q",
            Key::KeyW => "w",
            Key::KeyE => "e",
            Key::KeyR => "r",
            Key::KeyT => "t",
            Key::KeyY => "y",
            Key::KeyU => "u",
            Key::KeyI => "i",
            Key::KeyO => "o",
            Key::KeyP => "p",
            Key::LeftBracket => "LeftBracket",
            Key::RightBracket => "RightBracket",
            Key::KeyA => "a",
            Key::KeyS => "s",
            Key::KeyD => "d",
            Key::KeyF => "f",
            Key::KeyG => "g",
            Key::KeyH => "h",
            Key::KeyJ => "j",
            Key::KeyK => "k",
            Key::KeyL => "l",
            Key::SemiColon => "SemiColon",
            Key::Quote => "Quote",
            Key::BackSlash => "BackSlash",
            Key::IntlBackslash => "IntlBackslash",
            Key::KeyZ => "z",
            Key::KeyX => "x",
            Key::KeyC => "c",
            Key::KeyV => "v",
            Key::KeyB => "b",
            Key::KeyN => "n",
            Key::KeyM => "m",
            Key::Comma => "Comma",
            Key::Dot => "Dot",
            Key::Slash => "Slash",
            Key::Insert => "Insert",
            Key::KpReturn => "KpReturn",
            Key::KpMinus => "KpMinus",
            Key::KpPlus => "KpPlus",
            Key::KpMultiply => "KpMultiply",
            Key::KpDivide => "KpDivide",
            Key::Kp0 => "Kp0",
            Key::Kp1 => "Kp1",
            Key::Kp2 => "Kp2",
            Key::Kp3 => "Kp3",
            Key::Kp4 => "Kp4",
            Key::Kp5 => "Kp5",
            Key::Kp6 => "Kp6",
            Key::Kp7 => "Kp7",
            Key::Kp8 => "Kp8",
            Key::Kp9 => "Kp9",
            Key::KpDelete => "KpDelete",
            Key::Function => "Function",
            Key::Unknown(_) => "Unknown",
        }
    }
}
