use crate::mvc;
use crate::general_model;

pub enum Event {

}

pub struct Controller {
    model: general_model::Model,
    view: Box<dyn mvc::View<Event>>,
}

impl mvc::ViewListener<Event> for Controller {
    fn handle_event(&mut self, event: Event) {
        todo!()
    }
}

impl mvc::Controller<general_model::Model, Event> for Controller {
    fn start(&mut self) {
        todo!()
    }

    fn set_view(&mut self, view: Box<dyn mvc::View<Event>>) {
        todo!()
    }

    fn set_model(&mut self, model: general_model::Model) {
        todo!()
    }
}
