#[allow(unused_imports)]
use crate::i;
#[allow(unused_imports)]
use log::{info, warn};

use rdev::{
    Button as _Button, Event as _Event, EventType as _EventType, GrabError as _GrabError,
    Key as _Key, ListenError as _ListenError, SimulateError as _SimulateError,
};

use crate::{
    common::instant_elapse_millis,
    impls::{TraitInto, TraitReverseInto},
};

use super::{common::SimulateError, Button, Event, GrabError, Key, ListenError};

impl Into<Event> for _Event {
    fn into(self) -> Event {
        let elapse = instant_elapse_millis();
        match self.event_type {
            _EventType::KeyPress(key) => Event::KeyPress {
                key: key.into(),
                elapse,
                duration: 0,
            },
            _EventType::KeyRelease(key) => Event::KeyRelease {
                key: key.into(),
                elapse,
                duration: 0,
            },
            _EventType::ButtonPress { button, x, y } => Event::ButtonPress {
                button: button.into(),
                x,
                y,
                elapse,
                duration: 0,
            },
            _EventType::ButtonRelease { button, x, y } => Event::ButtonRelease {
                button: button.into(),
                x,
                y,
                elapse,
                duration: 0,
            },
            _EventType::Drag { button, x, y } => Event::Drag {
                button: button.into(),
                x,
                y,
                elapse,
                duration: 0,
            },
            _EventType::MouseMove { x, y } => Event::MouseMove {
                x,
                y,
                elapse,
                duration: 0,
            },
            _EventType::Wheel { delta_x, delta_y } => Event::Wheel {
                x: delta_x.into_diy(),
                y: delta_y.into_diy(),
                elapse,
                duration: 0,
            },
        }
    }
}

impl Into<_EventType> for Event {
    fn into(self) -> _EventType {
        match self {
            Event::KeyPress { key, .. } => _EventType::KeyPress(key.into()),
            Event::KeyRelease { key, .. } => _EventType::KeyRelease(key.into()),
            Event::ButtonPress { button, x, y, .. } => _EventType::ButtonPress {
                button: button.into(),
                x,
                y,
            },
            Event::ButtonRelease { button, x, y, .. } => _EventType::ButtonRelease {
                button: button.into(),
                x,
                y,
            },
            Event::MouseMove { x, y, .. } => _EventType::MouseMove { x, y },
            Event::Drag { button, x, y, .. } => _EventType::Drag {
                button: button.into(),
                x,
                y,
            },
            Event::Wheel { x, y, .. } => _EventType::Wheel {
                delta_x: x.into_std(),
                delta_y: y.into_std(),
            },
        }
    }
}

impl Into<GrabError> for _GrabError {
    fn into(self) -> GrabError {
        match self {
            _GrabError::EventTapError => GrabError::EventTapError,
            _GrabError::LoopSourceError => GrabError::LoopSourceError,
            _GrabError::MissingDisplayError => GrabError::MissingDisplayError,
            _GrabError::KeyboardError => GrabError::KeyboardError,
            _GrabError::KeyHookError(number) => GrabError::KeyHookError(number),
            _GrabError::MouseHookError(number) => GrabError::MouseHookError(number),
            _GrabError::SimulateError => GrabError::SimulateError,
            _GrabError::IoError(err) => GrabError::IoError(err),
            _ => todo!(),
        }
    }
}

impl Into<SimulateError> for _SimulateError {
    fn into(self) -> SimulateError {
        SimulateError::Default
    }
}

impl Into<ListenError> for _ListenError {
    fn into(self) -> ListenError {
        match self {
            _ListenError::EventTapError => ListenError::EventTapError,
            _ListenError::LoopSourceError => ListenError::LoopSourceError,
            _ListenError::MissingDisplayError => ListenError::MissingDisplayError,
            _ListenError::KeyboardError => ListenError::KeyboardError,
            _ListenError::RecordContextEnablingError => ListenError::RecordContextEnablingError,
            _ListenError::RecordContextError => ListenError::RecordContextError,
            _ListenError::XRecordExtensionError => ListenError::XRecordExtensionError,
            _ListenError::KeyHookError(number) => ListenError::KeyHookError(number),
            _ListenError::MouseHookError(number) => ListenError::MouseHookError(number),
            _ => todo!(),
        }
    }
}

impl Into<Key> for _Key {
    fn into(self) -> Key {
        match self {
            _Key::Alt => Key::Alt,
            _Key::AltGr => Key::AltGr,
            _Key::Backspace => Key::Backspace,
            _Key::CapsLock => Key::CapsLock,
            _Key::ControlLeft => Key::ControlLeft,
            _Key::ControlRight => Key::ControlRight,
            _Key::Delete => Key::Delete,
            _Key::DownArrow => Key::DownArrow,
            _Key::End => Key::End,
            _Key::Escape => Key::Escape,
            _Key::F1 => Key::F1,
            _Key::F10 => Key::F10,
            _Key::F11 => Key::F11,
            _Key::F12 => Key::F12,
            _Key::F2 => Key::F2,
            _Key::F3 => Key::F3,
            _Key::F4 => Key::F4,
            _Key::F5 => Key::F5,
            _Key::F6 => Key::F6,
            _Key::F7 => Key::F7,
            _Key::F8 => Key::F8,
            _Key::F9 => Key::F9,
            _Key::Home => Key::Home,
            _Key::LeftArrow => Key::LeftArrow,
            _Key::MetaLeft => Key::MetaLeft,
            _Key::MetaRight => Key::MetaRight,
            _Key::PageDown => Key::PageDown,
            _Key::PageUp => Key::PageUp,
            _Key::Return => Key::Return,
            _Key::RightArrow => Key::RightArrow,
            _Key::ShiftLeft => Key::ShiftLeft,
            _Key::ShiftRight => Key::ShiftRight,
            _Key::Space => Key::Space,
            _Key::Tab => Key::Tab,
            _Key::UpArrow => Key::UpArrow,
            _Key::PrintScreen => Key::PrintScreen,
            _Key::ScrollLock => Key::ScrollLock,
            _Key::Pause => Key::Pause,
            _Key::NumLock => Key::NumLock,
            _Key::BackQuote => Key::BackQuote,
            _Key::Num1 => Key::Num1,
            _Key::Num2 => Key::Num2,
            _Key::Num3 => Key::Num3,
            _Key::Num4 => Key::Num4,
            _Key::Num5 => Key::Num5,
            _Key::Num6 => Key::Num6,
            _Key::Num7 => Key::Num7,
            _Key::Num8 => Key::Num8,
            _Key::Num9 => Key::Num9,
            _Key::Num0 => Key::Num0,
            _Key::Minus => Key::Minus,
            _Key::Equal => Key::Equal,
            _Key::KeyQ => Key::KeyQ,
            _Key::KeyW => Key::KeyW,
            _Key::KeyE => Key::KeyE,
            _Key::KeyR => Key::KeyR,
            _Key::KeyT => Key::KeyT,
            _Key::KeyY => Key::KeyY,
            _Key::KeyU => Key::KeyU,
            _Key::KeyI => Key::KeyI,
            _Key::KeyO => Key::KeyO,
            _Key::KeyP => Key::KeyP,
            _Key::LeftBracket => Key::LeftBracket,
            _Key::RightBracket => Key::RightBracket,
            _Key::KeyA => Key::KeyA,
            _Key::KeyS => Key::KeyS,
            _Key::KeyD => Key::KeyD,
            _Key::KeyF => Key::KeyF,
            _Key::KeyG => Key::KeyG,
            _Key::KeyH => Key::KeyH,
            _Key::KeyJ => Key::KeyJ,
            _Key::KeyK => Key::KeyK,
            _Key::KeyL => Key::KeyL,
            _Key::SemiColon => Key::SemiColon,
            _Key::Quote => Key::Quote,
            _Key::BackSlash => Key::BackSlash,
            _Key::IntlBackslash => Key::IntlBackslash,
            _Key::KeyZ => Key::KeyZ,
            _Key::KeyX => Key::KeyX,
            _Key::KeyC => Key::KeyC,
            _Key::KeyV => Key::KeyV,
            _Key::KeyB => Key::KeyB,
            _Key::KeyN => Key::KeyN,
            _Key::KeyM => Key::KeyM,
            _Key::Comma => Key::Comma,
            _Key::Dot => Key::Dot,
            _Key::Slash => Key::Slash,
            _Key::Insert => Key::Insert,
            _Key::KpReturn => Key::KpReturn,
            _Key::KpMinus => Key::KpMinus,
            _Key::KpPlus => Key::KpPlus,
            _Key::KpMultiply => Key::KpMultiply,
            _Key::KpDivide => Key::KpDivide,
            _Key::Kp0 => Key::Kp0,
            _Key::Kp1 => Key::Kp1,
            _Key::Kp2 => Key::Kp2,
            _Key::Kp3 => Key::Kp3,
            _Key::Kp4 => Key::Kp4,
            _Key::Kp5 => Key::Kp5,
            _Key::Kp6 => Key::Kp6,
            _Key::Kp7 => Key::Kp7,
            _Key::Kp8 => Key::Kp8,
            _Key::Kp9 => Key::Kp9,
            _Key::KpDelete => Key::KpDelete,
            _Key::Function => Key::Function,
            _Key::Unknown(number) => Key::Unknown(number),
        }
    }
}

impl Into<_Key> for Key {
    fn into(self) -> _Key {
        match self {
            Key::Alt => _Key::Alt,
            Key::AltGr => _Key::AltGr,
            Key::Backspace => _Key::Backspace,
            Key::CapsLock => _Key::CapsLock,
            Key::ControlLeft => _Key::ControlLeft,
            Key::ControlRight => _Key::ControlRight,
            Key::Delete => _Key::Delete,
            Key::DownArrow => _Key::DownArrow,
            Key::End => _Key::End,
            Key::Escape => _Key::Escape,
            Key::F1 => _Key::F1,
            Key::F10 => _Key::F10,
            Key::F11 => _Key::F11,
            Key::F12 => _Key::F12,
            Key::F2 => _Key::F2,
            Key::F3 => _Key::F3,
            Key::F4 => _Key::F4,
            Key::F5 => _Key::F5,
            Key::F6 => _Key::F6,
            Key::F7 => _Key::F7,
            Key::F8 => _Key::F8,
            Key::F9 => _Key::F9,
            Key::Home => _Key::Home,
            Key::LeftArrow => _Key::LeftArrow,
            Key::MetaLeft => _Key::MetaLeft,
            Key::MetaRight => _Key::MetaRight,
            Key::PageDown => _Key::PageDown,
            Key::PageUp => _Key::PageUp,
            Key::Return => _Key::Return,
            Key::RightArrow => _Key::RightArrow,
            Key::ShiftLeft => _Key::ShiftLeft,
            Key::ShiftRight => _Key::ShiftRight,
            Key::Space => _Key::Space,
            Key::Tab => _Key::Tab,
            Key::UpArrow => _Key::UpArrow,
            Key::PrintScreen => _Key::PrintScreen,
            Key::ScrollLock => _Key::ScrollLock,
            Key::Pause => _Key::Pause,
            Key::NumLock => _Key::NumLock,
            Key::BackQuote => _Key::BackQuote,
            Key::Num1 => _Key::Num1,
            Key::Num2 => _Key::Num2,
            Key::Num3 => _Key::Num3,
            Key::Num4 => _Key::Num4,
            Key::Num5 => _Key::Num5,
            Key::Num6 => _Key::Num6,
            Key::Num7 => _Key::Num7,
            Key::Num8 => _Key::Num8,
            Key::Num9 => _Key::Num9,
            Key::Num0 => _Key::Num0,
            Key::Minus => _Key::Minus,
            Key::Equal => _Key::Equal,
            Key::KeyQ => _Key::KeyQ,
            Key::KeyW => _Key::KeyW,
            Key::KeyE => _Key::KeyE,
            Key::KeyR => _Key::KeyR,
            Key::KeyT => _Key::KeyT,
            Key::KeyY => _Key::KeyY,
            Key::KeyU => _Key::KeyU,
            Key::KeyI => _Key::KeyI,
            Key::KeyO => _Key::KeyO,
            Key::KeyP => _Key::KeyP,
            Key::LeftBracket => _Key::LeftBracket,
            Key::RightBracket => _Key::RightBracket,
            Key::KeyA => _Key::KeyA,
            Key::KeyS => _Key::KeyS,
            Key::KeyD => _Key::KeyD,
            Key::KeyF => _Key::KeyF,
            Key::KeyG => _Key::KeyG,
            Key::KeyH => _Key::KeyH,
            Key::KeyJ => _Key::KeyJ,
            Key::KeyK => _Key::KeyK,
            Key::KeyL => _Key::KeyL,
            Key::SemiColon => _Key::SemiColon,
            Key::Quote => _Key::Quote,
            Key::BackSlash => _Key::BackSlash,
            Key::IntlBackslash => _Key::IntlBackslash,
            Key::KeyZ => _Key::KeyZ,
            Key::KeyX => _Key::KeyX,
            Key::KeyC => _Key::KeyC,
            Key::KeyV => _Key::KeyV,
            Key::KeyB => _Key::KeyB,
            Key::KeyN => _Key::KeyN,
            Key::KeyM => _Key::KeyM,
            Key::Comma => _Key::Comma,
            Key::Dot => _Key::Dot,
            Key::Slash => _Key::Slash,
            Key::Insert => _Key::Insert,
            Key::KpReturn => _Key::KpReturn,
            Key::KpMinus => _Key::KpMinus,
            Key::KpPlus => _Key::KpPlus,
            Key::KpMultiply => _Key::KpMultiply,
            Key::KpDivide => _Key::KpDivide,
            Key::Kp0 => _Key::Kp0,
            Key::Kp1 => _Key::Kp1,
            Key::Kp2 => _Key::Kp2,
            Key::Kp3 => _Key::Kp3,
            Key::Kp4 => _Key::Kp4,
            Key::Kp5 => _Key::Kp5,
            Key::Kp6 => _Key::Kp6,
            Key::Kp7 => _Key::Kp7,
            Key::Kp8 => _Key::Kp8,
            Key::Kp9 => _Key::Kp9,
            Key::KpDelete => _Key::KpDelete,
            Key::Function => _Key::Function,
            Key::Unknown(number) => _Key::Unknown(number),
        }
    }
}

impl Into<Button> for _Button {
    fn into(self) -> Button {
        match self {
            _Button::Left => Button::Left,
            _Button::Right => Button::Right,
            _Button::Middle => Button::Middle,
            _Button::Unknown(number) => Button::Unknown(number),
        }
    }
}

impl Into<_Button> for Button {
    fn into(self) -> _Button {
        match self {
            Button::Left => _Button::Left,
            Button::Right => _Button::Right,
            Button::Middle => _Button::Middle,
            Button::Unknown(number) => _Button::Unknown(number),
        }
    }
}
