mod controller;
mod model;
mod view;

use controller::Controller;
use model::Model;
use view::View;

pub fn run() {
    let view = View::new();
    let model = Model {};
    let controller = Controller::new(view, model);
    controller.start();
}
