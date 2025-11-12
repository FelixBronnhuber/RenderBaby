mod controller;
mod model;
mod pipeline;
mod view;

use controller::Controller;
use model::Model;
use view::View;

pub fn run() {
    let pipeline = pipeline::Pipeline::new();
    let mut view = View::new(pipeline.clone());
    let model = Model::new();
    let controller = Controller::new(pipeline, model);
    view.set_listener(Box::new(controller));
    view.open();
}
