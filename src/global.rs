use lazy_static::lazy_static;
use rdev::{Button, Key};
use rhai::{Engine, Scope};
use std::io::Write;
use std::{
    fs::{self, OpenOptions},
    sync::Mutex,
    time::Instant,
};

use crate::common::*;

use crossbeam_channel::{unbounded, Receiver, Sender};

pub fn init_instant() {
    *NOW.lock().unwrap() = Some(Instant::now());
}

pub fn clean_instant() {
    *NOW.lock().unwrap() = None;
}

pub fn instant_elapse_millis() -> u128 {
    let now = NOW.lock().unwrap();
    if let Some(now) = *now {
        return now.elapsed().as_millis();
    }
    0
}

pub fn script_num() -> usize {
    let path = "scripts";
    let a = fs::read_dir(path).unwrap();
    a.count()
}

fn generate_script_filename() -> String {
    format!("undefined-{}.rhai", script_num() + 1)
}

pub fn generate_script_filepath() -> String {
    format!("scripts/{}", generate_script_filename())
}

pub fn write_to_file(filepath: &str, content: String) {
    let file = OpenOptions::new().append(true).create(true).open(filepath);
    if let Ok(mut file) = file {
        let _ = writeln!(file, "{}", content);
    }
}

pub fn run_rhai(filepath: &str) -> Result<(), Box<rhai::EvalAltResult>> {
    let mut engine = Engine::new();
    let mut scope = Scope::new();

    scope.push("ButtonLeft", Button::Left);
    scope.push("ButtonRight", Button::Right);

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
        .register_fn("drag_left_instant", drag_left_instant::<i32, i32>)
        .register_fn("drag_left_instant", drag_left_instant::<f64, f64>)
        .register_fn("drag_left_instant", drag_left_instant::<i32, f64>)
        .register_fn("drag_left_instant", drag_left_instant::<f64, i32>);
    engine
        .register_fn("drag_right_instant", drag_right_instant::<i32, i32>)
        .register_fn("drag_right_instant", drag_right_instant::<f64, f64>)
        .register_fn("drag_right_instant", drag_right_instant::<i32, f64>)
        .register_fn("drag_right_instant", drag_right_instant::<f64, i32>);
    engine
        .register_fn(
            "drag_left_relative",
            drag_left_relative::<i32, i32, i32, i32, i32>,
        )
        .register_fn(
            "drag_left_relative",
            drag_left_relative::<i32, i32, i32, i32, f64>,
        )
        .register_fn(
            "drag_left_relative",
            drag_left_relative::<i32, i32, i32, f64, i32>,
        )
        .register_fn(
            "drag_left_relative",
            drag_left_relative::<i32, i32, i32, f64, f64>,
        )
        .register_fn(
            "drag_left_relative",
            drag_left_relative::<i32, i32, f64, i32, i32>,
        )
        .register_fn(
            "drag_left_relative",
            drag_left_relative::<i32, i32, f64, i32, f64>,
        )
        .register_fn(
            "drag_left_relative",
            drag_left_relative::<i32, i32, f64, f64, i32>,
        )
        .register_fn(
            "drag_left_relative",
            drag_left_relative::<i32, i32, f64, f64, f64>,
        )
        .register_fn(
            "drag_left_relative",
            drag_left_relative::<i32, f64, i32, i32, i32>,
        )
        .register_fn(
            "drag_left_relative",
            drag_left_relative::<i32, f64, i32, i32, f64>,
        )
        .register_fn(
            "drag_left_relative",
            drag_left_relative::<i32, f64, i32, f64, i32>,
        )
        .register_fn(
            "drag_left_relative",
            drag_left_relative::<i32, f64, i32, f64, f64>,
        )
        .register_fn(
            "drag_left_relative",
            drag_left_relative::<i32, f64, f64, i32, i32>,
        )
        .register_fn(
            "drag_left_relative",
            drag_left_relative::<i32, f64, f64, i32, f64>,
        )
        .register_fn(
            "drag_left_relative",
            drag_left_relative::<i32, f64, f64, f64, i32>,
        )
        .register_fn(
            "drag_left_relative",
            drag_left_relative::<i32, f64, f64, f64, f64>,
        )
        .register_fn(
            "drag_left_relative",
            drag_left_relative::<f64, i32, i32, i32, i32>,
        )
        .register_fn(
            "drag_left_relative",
            drag_left_relative::<f64, i32, i32, i32, f64>,
        )
        .register_fn(
            "drag_left_relative",
            drag_left_relative::<f64, i32, i32, f64, i32>,
        )
        .register_fn(
            "drag_left_relative",
            drag_left_relative::<f64, i32, i32, f64, f64>,
        )
        .register_fn(
            "drag_left_relative",
            drag_left_relative::<f64, i32, f64, i32, i32>,
        )
        .register_fn(
            "drag_left_relative",
            drag_left_relative::<f64, i32, f64, i32, f64>,
        )
        .register_fn(
            "drag_left_relative",
            drag_left_relative::<f64, i32, f64, f64, i32>,
        )
        .register_fn(
            "drag_left_relative",
            drag_left_relative::<f64, i32, f64, f64, f64>,
        )
        .register_fn(
            "drag_left_relative",
            drag_left_relative::<f64, f64, i32, i32, i32>,
        )
        .register_fn(
            "drag_left_relative",
            drag_left_relative::<f64, f64, i32, i32, f64>,
        )
        .register_fn(
            "drag_left_relative",
            drag_left_relative::<f64, f64, i32, f64, i32>,
        )
        .register_fn(
            "drag_left_relative",
            drag_left_relative::<f64, f64, i32, f64, f64>,
        )
        .register_fn(
            "drag_left_relative",
            drag_left_relative::<f64, f64, f64, i32, i32>,
        )
        .register_fn(
            "drag_left_relative",
            drag_left_relative::<f64, f64, f64, i32, f64>,
        )
        .register_fn(
            "drag_left_relative",
            drag_left_relative::<f64, f64, f64, f64, i32>,
        )
        .register_fn(
            "drag_left_relative",
            drag_left_relative::<f64, f64, f64, f64, f64>,
        );
    engine
        .register_fn(
            "drag_right_relative",
            drag_right_relative::<i32, i32, i32, i32, i32>,
        )
        .register_fn(
            "drag_right_relative",
            drag_right_relative::<i32, i32, i32, i32, f64>,
        )
        .register_fn(
            "drag_right_relative",
            drag_right_relative::<i32, i32, i32, f64, i32>,
        )
        .register_fn(
            "drag_right_relative",
            drag_right_relative::<i32, i32, i32, f64, f64>,
        )
        .register_fn(
            "drag_right_relative",
            drag_right_relative::<i32, i32, f64, i32, i32>,
        )
        .register_fn(
            "drag_right_relative",
            drag_right_relative::<i32, i32, f64, i32, f64>,
        )
        .register_fn(
            "drag_right_relative",
            drag_right_relative::<i32, i32, f64, f64, i32>,
        )
        .register_fn(
            "drag_right_relative",
            drag_right_relative::<i32, i32, f64, f64, f64>,
        )
        .register_fn(
            "drag_right_relative",
            drag_right_relative::<i32, f64, i32, i32, i32>,
        )
        .register_fn(
            "drag_right_relative",
            drag_right_relative::<i32, f64, i32, i32, f64>,
        )
        .register_fn(
            "drag_right_relative",
            drag_right_relative::<i32, f64, i32, f64, i32>,
        )
        .register_fn(
            "drag_right_relative",
            drag_right_relative::<i32, f64, i32, f64, f64>,
        )
        .register_fn(
            "drag_right_relative",
            drag_right_relative::<i32, f64, f64, i32, i32>,
        )
        .register_fn(
            "drag_right_relative",
            drag_right_relative::<i32, f64, f64, i32, f64>,
        )
        .register_fn(
            "drag_right_relative",
            drag_right_relative::<i32, f64, f64, f64, i32>,
        )
        .register_fn(
            "drag_right_relative",
            drag_right_relative::<i32, f64, f64, f64, f64>,
        )
        .register_fn(
            "drag_right_relative",
            drag_right_relative::<f64, i32, i32, i32, i32>,
        )
        .register_fn(
            "drag_right_relative",
            drag_right_relative::<f64, i32, i32, i32, f64>,
        )
        .register_fn(
            "drag_right_relative",
            drag_right_relative::<f64, i32, i32, f64, i32>,
        )
        .register_fn(
            "drag_right_relative",
            drag_right_relative::<f64, i32, i32, f64, f64>,
        )
        .register_fn(
            "drag_right_relative",
            drag_right_relative::<f64, i32, f64, i32, i32>,
        )
        .register_fn(
            "drag_right_relative",
            drag_right_relative::<f64, i32, f64, i32, f64>,
        )
        .register_fn(
            "drag_right_relative",
            drag_right_relative::<f64, i32, f64, f64, i32>,
        )
        .register_fn(
            "drag_right_relative",
            drag_right_relative::<f64, i32, f64, f64, f64>,
        )
        .register_fn(
            "drag_right_relative",
            drag_right_relative::<f64, f64, i32, i32, i32>,
        )
        .register_fn(
            "drag_right_relative",
            drag_right_relative::<f64, f64, i32, i32, f64>,
        )
        .register_fn(
            "drag_right_relative",
            drag_right_relative::<f64, f64, i32, f64, i32>,
        )
        .register_fn(
            "drag_right_relative",
            drag_right_relative::<f64, f64, i32, f64, f64>,
        )
        .register_fn(
            "drag_right_relative",
            drag_right_relative::<f64, f64, f64, i32, i32>,
        )
        .register_fn(
            "drag_right_relative",
            drag_right_relative::<f64, f64, f64, i32, f64>,
        )
        .register_fn(
            "drag_right_relative",
            drag_right_relative::<f64, f64, f64, f64, i32>,
        )
        .register_fn(
            "drag_right_relative",
            drag_right_relative::<f64, f64, f64, f64, f64>,
        );
    engine
        .register_fn("click_left", click_left::<i32, i32>)
        .register_fn("click_left", click_left::<i32, f64>)
        .register_fn("click_left", click_left::<f64, i32>)
        .register_fn("click_left", click_left::<f64, f64>);
    engine
        .register_fn("click_right", click_right::<i32, i32>)
        .register_fn("click_right", click_right::<i32, f64>)
        .register_fn("click_right", click_right::<f64, i32>)
        .register_fn("click_right", click_right::<f64, f64>);
    engine.register_fn("button_left_click", button_left_click);
    engine.register_fn("button_right_click", button_right_click);
    engine
        .register_fn("mouse_move", mouse_move::<i32, i32>)
        .register_fn("mouse_move", mouse_move::<f64, f64>)
        .register_fn("mouse_move", mouse_move::<i32, f64>)
        .register_fn("mouse_move", mouse_move::<f64, i32>);
    engine
        .register_fn(
            "mouse_move_relative",
            mouse_move_relative::<i32, i32, i32, i32>,
        )
        .register_fn(
            "mouse_move_relative",
            mouse_move_relative::<i32, i32, i32, f64>,
        )
        .register_fn(
            "mouse_move_relative",
            mouse_move_relative::<i32, i32, f64, i32>,
        )
        .register_fn(
            "mouse_move_relative",
            mouse_move_relative::<i32, i32, f64, f64>,
        )
        .register_fn(
            "mouse_move_relative",
            mouse_move_relative::<i32, f64, i32, i32>,
        )
        .register_fn(
            "mouse_move_relative",
            mouse_move_relative::<i32, f64, i32, f64>,
        )
        .register_fn(
            "mouse_move_relative",
            mouse_move_relative::<i32, f64, f64, i32>,
        )
        .register_fn(
            "mouse_move_relative",
            mouse_move_relative::<i32, f64, f64, f64>,
        )
        .register_fn(
            "mouse_move_relative",
            mouse_move_relative::<f64, i32, i32, i32>,
        )
        .register_fn(
            "mouse_move_relative",
            mouse_move_relative::<f64, i32, i32, f64>,
        )
        .register_fn(
            "mouse_move_relative",
            mouse_move_relative::<f64, i32, f64, i32>,
        )
        .register_fn(
            "mouse_move_relative",
            mouse_move_relative::<f64, i32, f64, f64>,
        )
        .register_fn(
            "mouse_move_relative",
            mouse_move_relative::<f64, f64, i32, i32>,
        )
        .register_fn(
            "mouse_move_relative",
            mouse_move_relative::<f64, f64, i32, f64>,
        )
        .register_fn(
            "mouse_move_relative",
            mouse_move_relative::<f64, f64, f64, i32>,
        )
        .register_fn(
            "mouse_move_relative",
            mouse_move_relative::<f64, f64, f64, f64>,
        );
    engine
        .register_fn("wheel_up", wheel_up::<i32>)
        .register_fn("wheel_up", wheel_up::<f64>);
    engine
        .register_fn("wheel_down", wheel_down::<i32>)
        .register_fn("wheel_down", wheel_down::<f64>);
    engine
        .register_fn("delay", delay::<i32>)
        .register_fn("delay", delay::<f64>);
    engine.register_fn("key_press", key_press);
    engine.register_fn("key_release", key_release);
    engine.register_fn("key_click", key_click);
    engine.register_fn("paste_text", paste_text);
    engine.register_fn("select_all", select_all);
    engine
        .register_fn("button_left_press", button_left_press::<i32, i32>)
        .register_fn("button_left_press", button_left_press::<i32, f64>)
        .register_fn("button_left_press", button_left_press::<f64, i32>)
        .register_fn("button_left_press", button_left_press::<f64, f64>);
    engine
        .register_fn("button_left_release", button_left_release::<i32, i32>)
        .register_fn("button_left_release", button_left_release::<i32, f64>)
        .register_fn("button_left_release", button_left_release::<f64, i32>)
        .register_fn("button_left_release", button_left_release::<f64, f64>);
    engine
        .register_fn("button_right_press", button_right_press::<i32, i32>)
        .register_fn("button_right_press", button_right_press::<i32, f64>)
        .register_fn("button_right_press", button_right_press::<f64, i32>)
        .register_fn("button_right_press", button_right_press::<f64, f64>);
    engine
        .register_fn("button_right_release", button_right_release::<i32, i32>)
        .register_fn("button_right_release", button_right_release::<i32, f64>)
        .register_fn("button_right_release", button_right_release::<f64, i32>)
        .register_fn("button_right_release", button_right_release::<f64, f64>);
    engine.on_progress(move |_opt| loop {
        match STATE_CHANNEL.1.try_recv() {
            Ok(result) if result => {
                println!("I got stop message");
                return Some("stop".into());
            }
            Ok(_) => return None,
            Err(_) => return None,
        }
    });
    engine.run_file_with_scope(&mut scope, filepath.into())
}

pub fn state_send(msg: bool) {
    STATE_CHANNEL.0.send(msg).unwrap();
}

lazy_static! {
    pub static ref NOW: Mutex<Option<Instant>> = Mutex::new(None);
    static ref STATE_CHANNEL: (Sender<bool>, Receiver<bool>) = unbounded();
}
