mod controller;
mod model;
mod pipeline;
mod view;

use controller::Controller;
use model::Model;
use view::View;

pub struct App {
    view: View,
}

impl App {
    pub fn new() -> Self {
        App::create_egui_app()
    }

    pub fn create_egui_app() -> Self {
        let pipeline = pipeline::Pipeline::new();

        let model = Model::new();
        let controller = Controller::new(pipeline.clone(), model);
        let mut view = View::new(pipeline);

        view.set_listener(Box::new(controller));

        Self { view }
    }

    pub fn show(self) {
        self.view.open();
    }
}
