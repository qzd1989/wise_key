use std::{
    sync::{
        mpsc::{self, Receiver, Sender},
        Arc, Mutex,
    },
    thread::{self, JoinHandle},
};

type StateReceiveJob = Arc<Mutex<Receiver<bool>>>;

use device_query::{DeviceEvents, DeviceState};
use eframe::{
    egui::{self, Ui},
    AppCreator,
};
pub struct App {
    name: String,
    size: (f32, f32),
    pub position: Option<(i32, i32)>,
    pub state: bool,
    sender: Sender<bool>,
    receiver: StateReceiveJob,
    recorder: Option<Recorder>,
}

impl App {
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::channel();
        Self {
            name: "Wise Key".to_string(),
            size: (500.0, 500.0),
            position: Some((0, 0)),
            state: false,
            sender: sender,
            receiver: Arc::new(Mutex::new(receiver)),
            recorder: None,
        }
    }
    pub fn run() -> eframe::Result<()> {
        let app = Self::new();
        let name = app.name.clone();
        let native_options = eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default().with_inner_size([app.size.0, app.size.1]),
            ..Default::default()
        };
        let app_creator: AppCreator = Box::new(|_| Box::<App>::new(app));
        eframe::run_native(&name, native_options, app_creator)
    }
    pub fn ui(&mut self, ui: &mut Ui) {
        let label = if self.state { "Stop" } else { "Start" };
        if ui.button(label).clicked() {
            self.state = !self.state;
            if self.state {
                self.recorder = Some(Recorder::new(Arc::clone(&self.receiver)));
            }
            self.sender.send(self.state).unwrap();
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| self.ui(ui));
    }
}

struct Recorder {
    thread: Option<JoinHandle<()>>,
}

impl Recorder {
    fn new(receiver: StateReceiveJob) -> Self {
        let thread: JoinHandle<()> = thread::spawn(move || loop {
            let state = receiver.as_ref().lock().unwrap().recv().unwrap();
            let device_state: DeviceState = DeviceState::new();
            let guard = device_state.on_mouse_move(move |position| {
                println!("mouse position: {}:{}", position.0, position.1);
            });
            'inner: loop {
                //when state is false, I want to stop guard thread, but it seems I can't use state in this scope
                if !state {
                    drop(guard); //not working
                    break 'inner; //not working
                }
            }
        });
        Self {
            thread: Some(thread),
        }
    }
}
