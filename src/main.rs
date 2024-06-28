use device_query::{CallbackGuard, DeviceEvents, DeviceState, Keycode, MouseButton, MousePosition};
use eframe::{
    egui::{self, mutex::Mutex, Ui},
    AppCreator,
};
use lazy_static::lazy_static;
use std::time::Instant;

struct Data {
    current_position: (i32, i32),
    state: bool,
    actions: Mutex<Vec<Action>>,
}

trait BoolTrait {
    fn toggle(&mut self);
}

impl BoolTrait for bool {
    fn toggle(&mut self) {
        *self = !*self;
    }
}

#[derive(Debug)]
enum Action {
    MouseMove {
        x: i32,
        y: i32,
        elapse: u128,
        duration: u128,
    },
    MouseUp {
        x: i32,
        y: i32,
        button: MouseButton,
        elapse: u128,
        duration: u128,
    },
    MouseDown {
        x: i32,
        y: i32,
        button: MouseButton,
        elapse: u128,
        duration: u128,
    },
    KeyUp {
        keycode: Keycode,
        elapse: u128,
        duration: u128,
    },
    KeyDown {
        keycode: Keycode,
        elapse: u128,
        duration: u128,
    },
}

impl Action {
    fn elapse(&self) -> u128 {
        *match self {
            Action::MouseMove {
                x: _,
                y: _,
                elapse,
                duration: _,
            } => elapse,
            Action::MouseUp {
                x: _,
                y: _,
                button: _,
                elapse,
                duration: _,
            } => elapse,
            Action::MouseDown {
                x: _,
                y: _,
                button: _,
                elapse,
                duration: _,
            } => elapse,
            Action::KeyUp {
                keycode: _,
                elapse,
                duration: _,
            } => elapse,
            Action::KeyDown {
                keycode: _,
                elapse,
                duration: _,
            } => elapse,
        }
    }
}

impl Data {
    fn new() -> Self {
        let current_position = (0, 0);
        let state = false;
        let actions = Mutex::new(Vec::new());
        Self {
            current_position,
            state,
            actions,
        }
    }

    fn push(&mut self, action: Action) {
        let mut last_elapse = 0;
        let mut actions = self.actions.lock();
        let mut last_action: Option<&Action> = None;
        if actions.len() > 0 {
            last_action = Some(actions.last().unwrap());
            last_elapse = last_action.unwrap().elapse();
        }
        let now_elapsed = NOW.lock().elapsed().as_millis();
        let duration = now_elapsed - last_elapse;

        println!(
            "last action:{:?}, mills: {}, now: {}, duration: {}",
            last_action, last_elapse, now_elapsed, duration
        );

        let action = match action {
            Action::MouseMove {
                x,
                y,
                elapse,
                duration: _,
            } => Action::MouseMove {
                x,
                y,
                elapse,
                duration,
            },
            Action::MouseUp {
                x,
                y,
                button,
                elapse,
                duration: _,
            } => Action::MouseUp {
                x,
                y,
                button,
                elapse,
                duration,
            },
            Action::MouseDown {
                x,
                y,
                button,
                elapse,
                duration: _,
            } => Action::MouseDown {
                x,
                y,
                button,
                elapse,
                duration,
            },
            Action::KeyUp {
                keycode,
                elapse,
                duration: _,
            } => Action::KeyUp {
                keycode,
                elapse,
                duration,
            },
            Action::KeyDown {
                keycode,
                elapse,
                duration: _,
            } => Action::KeyDown {
                keycode,
                elapse,
                duration,
            },
        };
        actions.push(action);
    }
    fn clean(&mut self) {
        self.actions.lock().truncate(0);
    }
}

struct App {
    _data: &'static DATA,
}

impl App {
    fn new() -> Self {
        let _data = &DATA;
        Self { _data }
    }

    fn run() -> eframe::Result<()> {
        let app = Self::new();
        let native_options = eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default().with_inner_size([800.0, 500.0]),
            ..Default::default()
        };
        let app_creator: AppCreator = Box::new(|_| Box::<App>::new(app));
        eframe::run_native("Wise Key", native_options, app_creator)
    }

    pub fn ui(&mut self, ui: &mut Ui) {
        let current_position = DATA.lock().current_position;
        let label = format!("x:{},y:{}", current_position.0, current_position.1);
        if ui.button(label).clicked() {
            DATA.lock().state.toggle();
            let state = DATA.lock().state;
            if state {
                *NOW.lock() = Instant::now();
                *DEVICE_GUARD_MOUSE_MOVE.lock() =
                    Some(DEVICE_STATE.on_mouse_move(Box::new(|position| {
                        DATA.lock().current_position = (position.0, position.1);
                        let elapse = NOW.lock().elapsed().as_millis();
                        DATA.lock().push(Action::MouseMove {
                            x: position.0,
                            y: position.1,
                            elapse,
                            duration: 0,
                        });
                    })));
                *DEVICE_GUARD_MOUSE_UP.lock() =
                    Some(DEVICE_STATE.on_mouse_up(Box::new(|button| {
                        let current_position = DATA.lock().current_position;
                        let elapse = NOW.lock().elapsed().as_millis();
                        DATA.lock().push(Action::MouseUp {
                            x: current_position.0,
                            y: current_position.1,
                            button: *button,
                            elapse,
                            duration: 0,
                        });
                    })));

                *DEVICE_GUARD_MOUSE_DOWN.lock() =
                    Some(DEVICE_STATE.on_mouse_down(Box::new(|button| {
                        let current_position = DATA.lock().current_position;
                        let elapse = NOW.lock().elapsed().as_millis();
                        DATA.lock().push(Action::MouseDown {
                            x: current_position.0,
                            y: current_position.1,
                            button: *button,
                            elapse,
                            duration: 0,
                        });
                    })));
                *DEVICE_GUARD_KEY_UP.lock() = Some(DEVICE_STATE.on_key_up(Box::new(|keycode| {
                    let elapse = NOW.lock().elapsed().as_millis();
                    DATA.lock().push(Action::KeyUp {
                        keycode: *keycode,
                        elapse,
                        duration: 0,
                    });
                })));
                *DEVICE_GUARD_KEY_DOWN.lock() =
                    Some(DEVICE_STATE.on_key_down(Box::new(|keycode| {
                        let elapse = NOW.lock().elapsed().as_millis();
                        DATA.lock().push(Action::KeyDown {
                            keycode: *keycode,
                            elapse,
                            duration: 0,
                        });
                    })));
            } else {
                *DEVICE_GUARD_MOUSE_MOVE.lock() = None;
                *DEVICE_GUARD_MOUSE_UP.lock() = None;
                *DEVICE_GUARD_MOUSE_DOWN.lock() = None;
                *DEVICE_GUARD_KEY_UP.lock() = None;
                *DEVICE_GUARD_KEY_DOWN.lock() = None;
                DATA.lock().clean();
            }
        }
        let actions_vec: Vec<String> = DATA
            .lock()
            .actions
            .lock()
            .iter()
            .enumerate()
            .map(|(index, action)| format!("index:{}, action: {:?}", index, action))
            .rev()
            .collect();
        let action_text = actions_vec.join("\n");
        ui.label(action_text.as_str());
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| self.ui(ui));
    }
}

fn main() -> eframe::Result<()> {
    App::run()
}

lazy_static! {
    static ref DATA: Mutex<Data> = Mutex::new(Data::new());
    static ref DEVICE_STATE: DeviceState = DeviceState::new();
    static ref DEVICE_GUARD_MOUSE_MOVE: Mutex<Option<CallbackGuard<Box<dyn Fn(&MousePosition) + Sync + Send + 'static>>>> =
        Mutex::new(None);
    static ref DEVICE_GUARD_MOUSE_UP: Mutex<Option<CallbackGuard<Box<dyn Fn(&MouseButton) + Sync + Send + 'static>>>> =
        Mutex::new(None);
    static ref DEVICE_GUARD_MOUSE_DOWN: Mutex<Option<CallbackGuard<Box<dyn Fn(&MouseButton) + Sync + Send + 'static>>>> =
        Mutex::new(None);
    static ref DEVICE_GUARD_KEY_UP: Mutex<Option<CallbackGuard<Box<dyn Fn(&Keycode) + Sync + Send + 'static>>>> =
        Mutex::new(None);
    static ref DEVICE_GUARD_KEY_DOWN: Mutex<Option<CallbackGuard<Box<dyn Fn(&Keycode) + Sync + Send + 'static>>>> =
        Mutex::new(None);
    static ref NOW: Mutex<Instant> = Mutex::new(Instant::now());
}
