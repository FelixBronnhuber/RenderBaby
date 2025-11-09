use crate::mvc;
use crate::general_controller;
use crate::mvc::ViewListener;

pub struct EGuiView {
    listener: Option<Box<dyn mvc::ViewListener<general_controller::Event>>>,
}

impl EGuiView {
    pub fn new() -> Self {
        Self { listener: None }
    }
}

impl mvc::View<general_controller::Event> for EGuiView {
    fn run(self) {
        todo!()
    }

    fn set_listener(&mut self, listener: Box<dyn ViewListener<general_controller::Event>>) {
        self.listener = Some(listener);
    }
}
