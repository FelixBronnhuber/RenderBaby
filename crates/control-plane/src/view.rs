use eframe::{App, Frame};
use eframe::egui::Context;

pub enum Event {

}

pub trait ViewListener {
    fn handle_event(&mut self, event: Event);
}

pub struct View {
    listener: Option<Box<dyn ViewListener>>,
}

impl App for View {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        // ...
    }
}

impl View {
    pub fn new() -> Self {
        View { listener: None }
    }

    pub fn open(self) {
        let options = eframe::NativeOptions::default();
        let _ = eframe::run_native(
            "RenderBaby",
            options,
            Box::new(|_cc| Ok(Box::new(self))),
        );
    }

    pub fn set_listener(&mut self, listener: Box<dyn ViewListener>) {
        self.listener = Some(listener);
    }
}