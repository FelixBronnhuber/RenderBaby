use crate::view::View;
use crate::model::EGuiModel;


pub trait EGuiViewListener {

}

pub struct EGuiView {
    view_listener: Option<Box<dyn EGuiViewListener>>,
}

impl EGuiView {
    pub fn new() -> Self {
        Self
    }

    fn set_listener(&mut self, listener: Box<dyn EGuiViewListener>) {

    }
}

impl View<EGuiModel> for EGuiView {
    fn run(&mut self) {

    }

    fn update(&mut self, model: &EGuiModel) {

    }
}