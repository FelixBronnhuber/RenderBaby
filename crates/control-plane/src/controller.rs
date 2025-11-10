use crate::view::*;
use crate::model::*;

pub struct Controller {
    view: Option<View>,
    model: Model,
}

impl Controller {

    pub fn new(view: View, model: Model) -> Self {
        Self {
            view: Some(view),
            model,
        }
    }

    pub fn start(mut self) {
        let mut view = self.view.take().expect("view already taken");
        view.set_listener(Box::new(self));
        view.open();
    }
}

impl ViewListener for Controller {
    fn handle_event(&mut self, event: Event) {
        todo!()
    }
}
