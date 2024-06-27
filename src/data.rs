use std::{
    sync::{
        mpsc::{self, Sender},
        Arc, Mutex,
    },
    thread::{self, JoinHandle},
};

use device_query::{DeviceEvents, DeviceState};

use eframe::{
    egui::{self, Ui},
    AppCreator,
};
pub struct App {
    name: String,
    size: (f32, f32),
    state: bool,
    sender: Sender<bool>,
    recored_thread: JoinHandle<()>,
}

impl App {
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let recored_receiver = Arc::clone(&receiver);
        let recored_thread = thread::spawn(move || loop {
            let state = recored_receiver.as_ref().lock().unwrap().recv().unwrap();
            println!("state is {}", state);
        });
        Self {
            name: "Wise Key".to_string(),
            size: (500.0, 500.0),
            state: false,
            sender,
            recored_thread,
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
            self.sender.send(self.state).unwrap();
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| self.ui(ui));
    }
}
