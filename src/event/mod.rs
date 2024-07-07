#[allow(unused_imports)]
use crate::i;
#[allow(unused_imports)]
use log::{info, warn};

use crate::{
    common::{Float, Int, UInt},
    impls::TraitReverseInto,
};
use std::{thread::sleep, time::Duration};
mod common;
mod impls;
mod rhai;
pub use common::{
    current_point, events_to_data, grab, simulate_event, simulate_event_data, GrabError,
    ListenError, SimulateError,
};

#[derive(Debug, Clone, Copy)]
pub enum Event {
    KeyPress {
        key: Key,
        elapse: UInt,
        duration: UInt,
    },
    KeyRelease {
        key: Key,
        elapse: UInt,
        duration: UInt,
    },
    ButtonPress {
        button: Button,
        x: Float,
        y: Float,
        elapse: UInt,
        duration: UInt,
    },
    ButtonRelease {
        button: Button,
        x: Float,
        y: Float,
        elapse: UInt,
        duration: UInt,
    },
    MouseMove {
        x: Float,
        y: Float,
        elapse: UInt,
        duration: UInt,
    },
    Drag {
        button: Button,
        x: Float,
        y: Float,
        elapse: UInt,
        duration: UInt,
    },
    Wheel {
        x: Int,
        y: Int,
        elapse: UInt,
        duration: UInt,
    },
}

impl Event {
    pub fn build(event: Event, previous_event: Option<&Event>) -> Event {
        match previous_event {
            Some(previous_event) => {
                let mut duration = 0;
                if event.elapse() > previous_event.elapse() {
                    duration = event.elapse() - previous_event.elapse();
                }
                event.set_duration(duration)
            }
            None => event,
        }
    }
    fn set_duration(mut self, value: UInt) -> Self {
        match self {
            Event::KeyPress {
                ref mut duration, ..
            } => *duration = value,
            Event::KeyRelease {
                ref mut duration, ..
            } => *duration = value,
            Event::ButtonPress {
                ref mut duration, ..
            } => *duration = value,
            Event::ButtonRelease {
                ref mut duration, ..
            } => *duration = value,
            Event::MouseMove {
                ref mut duration, ..
            } => *duration = value,
            Event::Drag {
                ref mut duration, ..
            } => *duration = value,
            Event::Wheel {
                ref mut duration, ..
            } => *duration = value,
        };
        self
    }
    fn elapse(&self) -> UInt {
        match self {
            Event::KeyPress { elapse, .. } => *elapse,
            Event::KeyRelease { elapse, .. } => *elapse,
            Event::ButtonPress { elapse, .. } => *elapse,
            Event::ButtonRelease { elapse, .. } => *elapse,
            Event::MouseMove { elapse, .. } => *elapse,
            Event::Drag { elapse, .. } => *elapse,
            Event::Wheel { elapse, .. } => *elapse,
        }
    }

    fn duration(&self) -> UInt {
        match self {
            Event::KeyPress { duration, .. } => *duration,
            Event::KeyRelease { duration, .. } => *duration,
            Event::ButtonPress { duration, .. } => *duration,
            Event::ButtonRelease { duration, .. } => *duration,
            Event::MouseMove { duration, .. } => *duration,
            Event::Drag { duration, .. } => *duration,
            Event::Wheel { duration, .. } => *duration,
        }
    }

    pub fn to_string(&self) -> String {
        let delay_str: String = {
            let duration = self.duration();
            let mut delay_str = String::from("\n");
            if duration > 0 {
                delay_str += &format!("delay({});\n", duration);
            }
            delay_str
        };
        match self {
            Event::KeyPress { key, .. } => {
                format!("key_press({});{}", key.as_str(), delay_str)
            }
            Event::KeyRelease { key, .. } => {
                format!("key_release({});{}", key.as_str(), delay_str)
            }
            Event::ButtonPress { button, x, y, .. } => match button {
                Button::Left => format!("button_left_press({},{});{}", x, y, delay_str),
                Button::Right => format!("button_right_press({},{});{}", x, y, delay_str),
                Button::Middle => String::new(),
                Button::Unknown(_) => String::new(),
            },
            Event::ButtonRelease { button, x, y, .. } => match button {
                Button::Left => format!("button_left_release({},{});{}", x, y, delay_str),
                Button::Right => format!("button_right_release({},{});{}", x, y, delay_str),
                Button::Middle => String::new(),
                Button::Unknown(_) => String::new(),
            },
            Event::MouseMove { x, y, .. } => format!("mouse_move({},{});{}", x, y, delay_str),
            Event::Drag { button, x, y, .. } => match button {
                Button::Left => format!("drag_left_instant({},{});{}", x, y, delay_str),
                Button::Right => format!("drag_right_instant({},{});{}", x, y, delay_str),
                Button::Middle => String::new(),
                Button::Unknown(_) => String::new(),
            },
            Event::Wheel { y, .. } => {
                if *y > 0 {
                    format!("wheel_down({});\ndelay({});", y, delay_str)
                } else if *y < 0 {
                    format!("wheel_up({});\ndelay({});", y.abs(), delay_str)
                } else {
                    String::new()
                }
            }
        }
    }

    fn simulate(&self) -> Result<(), SimulateError> {
        sleep(Duration::from_millis(self.duration().into_std()));
        //info!("simulate: {:?}", self);
        let event = self.clone();
        simulate_event(event)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Key {
    Alt,
    AltGr,
    Backspace,
    CapsLock,
    ControlLeft,
    ControlRight,
    Delete,
    DownArrow,
    End,
    Escape,
    F1,
    F10,
    F11,
    F12,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    Home,
    LeftArrow,
    /// also known as "windows", "super", and "command"
    MetaLeft,
    /// also known as "windows", "super", and "command"
    MetaRight,
    PageDown,
    PageUp,
    Return,
    RightArrow,
    ShiftLeft,
    ShiftRight,
    Space,
    Tab,
    UpArrow,
    PrintScreen,
    ScrollLock,
    Pause,
    NumLock,
    BackQuote,
    Num1,
    Num2,
    Num3,
    Num4,
    Num5,
    Num6,
    Num7,
    Num8,
    Num9,
    Num0,
    Minus,
    Equal,
    KeyQ,
    KeyW,
    KeyE,
    KeyR,
    KeyT,
    KeyY,
    KeyU,
    KeyI,
    KeyO,
    KeyP,
    LeftBracket,
    RightBracket,
    KeyA,
    KeyS,
    KeyD,
    KeyF,
    KeyG,
    KeyH,
    KeyJ,
    KeyK,
    KeyL,
    SemiColon,
    Quote,
    BackSlash,
    IntlBackslash,
    KeyZ,
    KeyX,
    KeyC,
    KeyV,
    KeyB,
    KeyN,
    KeyM,
    Comma,
    Dot,
    Slash,
    Insert,
    KpReturn,
    KpMinus,
    KpPlus,
    KpMultiply,
    KpDivide,
    Kp0,
    Kp1,
    Kp2,
    Kp3,
    Kp4,
    Kp5,
    Kp6,
    Kp7,
    Kp8,
    Kp9,
    KpDelete,
    Function,
    Unknown(UInt),
}

impl Key {
    pub fn as_str(&self) -> &str {
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

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub enum Button {
    Left,
    Right,
    Middle,
    Unknown(u8),
}

#[derive(Debug, Clone)]
pub struct Data {
    name: String,
    resolution: (Int, Int),
    os: String,
    os_version: String,
    pub content: String,
}
impl Data {
    pub fn new(name: String, content: String) -> Self {
        Self {
            name,
            resolution: (0, 0),
            os: String::new(),
            os_version: String::new(),
            content,
        }
    }
    pub fn simulate(&self) -> Result<(), SimulateError> {
        simulate_event_data(self)
    }
}
