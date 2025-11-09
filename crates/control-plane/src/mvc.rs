pub trait Model {
    fn update(&mut self); // just general, might not be needed.

    /* functions that communicate with other planes ... */
}

pub trait ViewListener<E> {
    fn handle_event(&mut self, event: E);
}

pub trait View<E> {
    fn run(self);
    fn set_listener(&mut self, listener: Box<dyn ViewListener<E>>);
}

pub trait Controller<M: Model, E> : ViewListener<E> {
    fn start(&mut self);
    fn set_view(&mut self, view: Box<dyn View<E>>);
    fn set_model(&mut self, model: M);
}
