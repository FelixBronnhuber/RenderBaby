use crate::model::Model;


pub trait ViewListener {
    /* functions... */
}

pub trait View<M: Model> {
    fn run(&mut self, listener: &ViewListener);
    fn update(&mut self, model: &M);
    fn set_listener(&mut self, listener: Box<dyn ViewListener>);
}
