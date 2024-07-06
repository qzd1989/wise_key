use crate::{
    event::Event,
    global::{
        clean_instant, generate_script_filepath, init_instant, run_rhai, state_send, write_to_file,
    },
};
use rdev::{
    grab as _grab, listen as _listen, stop_listen as _stop_listen, Event as _Event, EventType, Key,
};
use std::{
    sync::{Arc, Mutex},
    thread::spawn,
};

#[derive(Clone, Copy)]
struct HotKey {
    stop: Key,
    record: Key,
    execute: Key,
}

impl HotKey {
    fn new(record: Key, execute: Key, stop: Key) -> Self {
        Self {
            record,
            execute,
            stop,
        }
    }
    fn contains(&self, key: &Key) -> bool {
        let keys = vec![self.record, self.execute, self.stop];
        keys.contains(key)
    }
}

#[derive(Debug, PartialEq)]
enum State {
    Stop,
    Record,
    Execute,
}

impl State {
    fn default() -> Self {
        Self::Stop
    }
}

#[derive(Clone, Copy)]
#[allow(dead_code)]
enum LoopTimes {
    Infinite, //why should I use dead_code?  variant `Infinite` is never constructed `LoopTime` has a derived impl for the trait `Clone`, but this is intentionally ignored during dead code analysis `#[warn(dead_code)]` on by defaultrustcClick for full compiler diagnostic
    Finite(u32),
}

pub struct App {
    state: State,
    loop_times: LoopTimes,
    events: Vec<Event>,
    mouse_move_filter: bool,
    hotkeys: HotKey,
    filepath: Option<String>,
}

impl App {
    pub fn new() -> Self {
        Self {
            state: State::default(),
            events: Vec::new(),
            mouse_move_filter: false,
            loop_times: LoopTimes::Finite(1),
            hotkeys: HotKey::new(Key::Num1, Key::Num2, Key::Num3),
            filepath: Some(String::from("scripts/example.rhai")),
        }
    }

    pub fn run(self) -> Arc<Mutex<Self>> {
        let app = Arc::new(Mutex::new(self));
        let app_clone = Arc::clone(&app);
        grab(app_clone);
        app
    }

    fn push(&mut self, event: Event) {
        self.events.push(Event::build(event, self.events.last()));
    }

    pub fn record(&mut self, callback: Box<dyn FnOnce() + 'static>) {
        self.state = State::Record;
        self.events.truncate(0);
        init_instant();
        callback();
    }

    pub fn stop(&mut self) {
        self.state = State::Stop;
        clean_instant();
    }

    pub fn save_to_file(&mut self) {
        self.filepath = Some(generate_script_filepath());
        if let Some(filepath) = &self.filepath {
            self.events.iter().for_each(|event| {
                write_to_file(filepath, event.to_rhai());
            });
        }
    }

    fn execute(&mut self, callback: Box<dyn FnOnce(String) + 'static>) {
        self.state = State::Execute;
        if let Some(filepath) = &self.filepath {
            callback(String::from(filepath.clone()));
        }
    }
}

fn grab(app: Arc<Mutex<App>>) {
    let app_clone_for_hotkeys = Arc::clone(&app);
    let hotkey = app_clone_for_hotkeys.lock().unwrap().hotkeys.clone();
    spawn(move || {
        if let Err(error) = _grab(move |event| match event.event_type {
            EventType::KeyRelease(key) if key == hotkey.record => {
                println!("recording");
                let mut app_lock = app.lock().unwrap();
                if app_lock.state != State::Stop {
                    return Some(event);
                }
                let app_clone = Arc::clone(&app);
                let record_callback = Box::new(move || {
                    spawn(move || {
                        _listen(move |event| {
                            let app_clone = Arc::clone(&app_clone);
                            callback(event, app_clone);
                        })
                    });
                });
                app_lock.record(record_callback);
                Some(event)
            }
            EventType::KeyRelease(key) if key == hotkey.execute => {
                println!("executing");
                let mut app_lock = app.lock().unwrap();
                if app_lock.state != State::Stop {
                    return Some(event);
                }
                let loop_times = app_lock.loop_times.clone();
                let app_execute = Arc::clone(&app);
                let app_single_operation = Arc::clone(&app);
                let app_state = Arc::clone(&app);
                let single = move |filepath: String| {
                    println!("executing");
                    if let Err(err) = run_rhai(filepath.as_str()) {
                        println!("run rhai error: {:?}", err);
                        app_single_operation.lock().unwrap().stop();
                    }
                };
                let execute_callback = Box::new(move |filepath: String| {
                    spawn(move || {
                        match loop_times {
                            LoopTimes::Infinite => {
                                while app_state.lock().unwrap().state != State::Stop {
                                    single(filepath.clone());
                                }
                            }
                            LoopTimes::Finite(times) => {
                                for _ in 0..times {
                                    single(filepath.clone());
                                }
                            }
                        }
                        app_execute.lock().unwrap().stop();
                    });
                });
                app_lock.execute(execute_callback);
                None
            }
            EventType::KeyRelease(key) if key == hotkey.stop => {
                println!("stopping");
                let mut app_lock = app.lock().unwrap();
                if app_lock.state == State::Record {
                    app_lock.save_to_file();
                }
                if app_lock.state == State::Execute {
                    state_send(true);
                }
                if app_lock.state == State::Stop {
                    return Some(event);
                }
                app_lock.stop();
                spawn(|| {
                    _stop_listen();
                });
                Some(event)
            }
            _ => Some(event),
        }) {
            println!("grab error: {:?}", error);
        }
    });
}

fn callback(event: _Event, app: Arc<Mutex<App>>) {
    let mut app = app.lock().unwrap();
    let event = event.into();
    match (app.mouse_move_filter, event) {
        (_, Event::KeyPress { key, .. }) if app.hotkeys.contains(&key) => {
            //hotkeys won't be pushed
        }
        (_, Event::KeyRelease { key, .. }) if app.hotkeys.contains(&key) => {
            //hotkeys won't be pushed
        }
        (true, Event::MouseMove { .. }) => {
            //MouseMove won't be pushed if mouse_move_filter is true
        }
        _ => {
            println!("pushing: {:?}", event);
            app.push(event);
        }
    }
}
