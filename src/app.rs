#[allow(unused_imports)]
use crate::i;
#[allow(unused_imports)]
use log::{info, warn};

use std::{
    sync::{Arc, RwLock},
    thread::{spawn, JoinHandle},
};

use crate::{
    common::{clean_instant, init_instant, simulate_state_send, Int},
    event::{events_to_data, grab, Data, Event, Key},
};

pub struct App {
    hotkey: Arc<RwLock<HotKey>>,
    loop_times: Arc<RwLock<LoopTimes>>,
    mouse_filter: Arc<RwLock<bool>>,
    events: Arc<RwLock<Vec<Event>>>,
    data: Arc<RwLock<Option<Data>>>,
    state: Arc<RwLock<State>>,
    grab_handle: Option<JoinHandle<()>>,
}
impl App {
    pub fn new() -> Self {
        let mut app = Self {
            hotkey: Arc::new(RwLock::new(HotKey::default())),
            mouse_filter: Arc::new(RwLock::new(false)),
            loop_times: Arc::new(RwLock::new(LoopTimes::Limited(1))),
            state: Arc::new(RwLock::new(State::default())),
            events: Arc::new(RwLock::new(Vec::new())),
            data: Arc::new(RwLock::new(None)),
            grab_handle: None,
        };
        let hotkey = Arc::clone(&app.hotkey);
        let mouse_filter = Arc::clone(&app.mouse_filter);
        let loop_times = Arc::clone(&app.loop_times);
        let state = Arc::clone(&app.state);
        let events = Arc::clone(&app.events);
        let data: Arc<RwLock<Option<Data>>> = Arc::clone(&app.data);
        app.grab_handle = Some(spawn(move || {
            Self::_grab(hotkey, mouse_filter, loop_times, state, events, data)
        }));
        app
    }
    pub fn run(&self) {}
    fn _grab(
        hotkey: Arc<RwLock<HotKey>>,
        mouse_filter: Arc<RwLock<bool>>,
        loop_times: Arc<RwLock<LoopTimes>>,
        state: Arc<RwLock<State>>,
        events: Arc<RwLock<Vec<Event>>>,
        data: Arc<RwLock<Option<Data>>>,
    ) {
        let state_clone = Arc::clone(&state);
        if let Err(_) = grab(move |_event| {
            let hotkey = Arc::clone(&hotkey);
            let mouse_filter = Arc::clone(&mouse_filter);
            let loop_times = Arc::clone(&loop_times);
            let state = Arc::clone(&state);
            let events_stop = Arc::clone(&events);
            let events_push = Arc::clone(&events);
            let data = Arc::clone(&data);
            let event: Event = _event.clone().into();
            match event {
                Event::KeyPress { key, .. } if hotkey.read().unwrap().contains(&key) => None,
                Event::KeyRelease { key, .. } if key == hotkey.read().unwrap().record => {
                    Self::record(state);
                    None
                }
                Event::KeyRelease { key, .. } if key == hotkey.read().unwrap().simulate => {
                    Self::simulate(state, loop_times, data);
                    None
                }
                Event::KeyRelease { key, .. } if key == hotkey.read().unwrap().stop => {
                    Self::stop(state, events_stop, data);
                    None
                }
                _ => {
                    Self::_push(state, mouse_filter, event, events_push);
                    Some(_event)
                }
            }
        }) {
            //show error to user
        } else {
            *state_clone.write().unwrap() = State::Stop;
        }
    }
    fn record(state: Arc<RwLock<State>>) {
        info!("recording");
        if *state.write().unwrap() != State::Stop {
            return;
        }
        *state.write().unwrap() = State::Record;
        init_instant();
    }
    fn stop(
        state: Arc<RwLock<State>>,
        events: Arc<RwLock<Vec<Event>>>,
        data: Arc<RwLock<Option<Data>>>,
    ) {
        info!("stopping");
        if *state.write().unwrap() == State::Stop {
            return;
        }
        clean_instant();
        let previous_state = state.read().unwrap().clone();
        *state.write().unwrap() = State::Stop;
        let len = events.read().unwrap().len().clone();
        match (previous_state, len > 0) {
            (State::Record, true) => {
                *data.write().unwrap() = Some(events_to_data(&*events.read().unwrap()));
                *events.write().unwrap() = Vec::new();
            }
            (State::Simulate, _) => {
                simulate_state_send(true);
            }
            (_, _) => {}
        };
        let guard = data.read().unwrap();
        if let Some(ref data) = *guard {
            info!("data: {:?}", data.content);
        }
    }

    fn simulate(
        state: Arc<RwLock<State>>,
        loop_times: Arc<RwLock<LoopTimes>>,
        data: Arc<RwLock<Option<Data>>>,
    ) {
        info!("simulating");
        if *state.write().unwrap() != State::Stop {
            return;
        }
        spawn(move || {
            *state.write().unwrap() = State::Simulate;
            let data_guard = data.read().unwrap();
            if let Some(ref data) = *data_guard {
                let data = data.clone();
                let state = Arc::clone(&state);
                //在一个线程的话无法继续监听hotkey
                spawn(move || {
                    match *loop_times.read().unwrap() {
                        LoopTimes::Unlimited => {
                            while *state.read().unwrap() != State::Stop {
                                if let Err(_) = data.simulate() {
                                    //show error to user
                                    break;
                                }
                            }
                        }
                        LoopTimes::Limited(times) => {
                            for _ in 0..times {
                                if let Err(_) = data.simulate() {
                                    //show error to user
                                    break;
                                }
                            }
                        }
                    };
                    //stop
                    *state.write().unwrap() = State::Stop;
                });
            } else {
                *state.write().unwrap() = State::Stop;
            }
        });
    }
    fn _push(
        state: Arc<RwLock<State>>,
        mouse_filter: Arc<RwLock<bool>>,
        event: Event,
        events: Arc<RwLock<Vec<Event>>>,
    ) {
        match *state.read().unwrap() {
            State::Record => match (*mouse_filter.read().unwrap(), event) {
                (true, Event::MouseMove { .. }) => {}
                _ => {
                    let event = Event::build(event, events.read().unwrap().last());
                    match events.try_write() {
                        Ok(mut events) => {
                            info!("pushing {:?}", event);
                            events.push(event);
                        }
                        Err(err) => warn!("{:?}", err),
                    };
                }
            },
            _ => {}
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum State {
    Stop,
    Record,
    Simulate,
}

impl State {
    fn default() -> Self {
        Self::Stop
    }
}

#[derive(Clone, Copy)]
struct HotKey {
    pub stop: Key,
    pub record: Key,
    pub simulate: Key,
}

impl HotKey {
    fn default() -> Self {
        Self {
            record: Key::F10,
            simulate: Key::F11,
            stop: Key::F12,
        }
    }
    fn contains(&self, key: &Key) -> bool {
        let keys = vec![self.record, self.simulate, self.stop];
        keys.contains(key)
    }
}

#[derive(Clone, Copy)]
#[allow(dead_code)]
enum LoopTimes {
    Unlimited,
    Limited(Int),
}
