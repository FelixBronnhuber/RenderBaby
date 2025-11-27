use mvc_view_wrappers::ViewWrapper;

mod controller;
mod model;
mod pipeline;
mod view;

pub struct App {
    view: view::View,
}

impl App {
    pub fn new() -> Self {
        Self::new_eframe_app()
    }

    fn new_eframe_app() -> Self {
        let pipeline = pipeline::Pipeline::new();
        let mut controller = controller::Controller::new(model::Model::new(), pipeline.clone());

        let handler = Box::new(move |event| controller.handle_event(event));

        let view = view::View::new(pipeline, handler);

        App { view }
    }

    pub fn show(self) {
        self.view.open();
    }
}
