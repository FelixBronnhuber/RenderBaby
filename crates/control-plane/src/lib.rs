pub mod controller;
pub mod model;
pub mod view;

use crate::controller::Controller;
use crate::model::Model;
use crate::view::View;
use std::sync::{Arc, Mutex};
use crossbeam::channel::unbounded;
use eframe::NativeOptions;

pub fn run() {
    let (tx, rx) = unbounded();

    let model = Model::new();
    let controller = Arc::new(Mutex::new(Controller::new(model, tx, 512, 512)));

    let mut view = View::new(rx);
    view.set_listener(controller);

    let options = NativeOptions::default();
    let _ = eframe::run_native(
        "RenderBaby",
        options,
        Box::new(|_cc| Ok(Box::new(view))),
    );
}
