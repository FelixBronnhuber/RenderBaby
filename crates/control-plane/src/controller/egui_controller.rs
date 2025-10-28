use crate::controller::Controller;
use crate::model::EGuiModel;
use crate::view::{EGuiViewListener, EGuiView};

pub struct EGuiController {
    model: EGuiModel,
    view: EGuiView,
}

impl EGuiController {
    pub fn new(model: EGuiModel, mut view: EGuiView) -> Self {
        Self
    }

    pub fn update(&mut self) {

    }
}

impl EGuiViewListener for EGuiController {

}

impl Controller<EGuiModel, EGuiView> for EGuiController {
    fn new (model: EGuiModel, mut view: EGuiView) -> Self {
        Self
    }

    fn update_cycle(&mut self) {

    }
}
