use view_wrappers::ViewWrapper;
use crate::control_plane::app::App;

pub mod controller;
pub mod model;
pub mod pipeline;
pub mod view;

pub struct GuiApp {
    view: view::View,
}

impl App for GuiApp {
    fn new() -> Self {
        let pipeline = pipeline::Pipeline::new();
        let mut controller = controller::Controller::new(model::Model::new(), pipeline.clone());

        let handler = Box::new(move |event| controller.handle_event(event));

        let view = view::View::new(pipeline, handler);

        GuiApp { view }
    }

    fn show(self: Box<GuiApp>) {
        self.view.open();
    }
}
