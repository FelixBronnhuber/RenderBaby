pub trait Model {
    fn update(&mut self); // just general, might not be needed.

    /* functions that communicate with other planes ... */
}

pub trait ViewListener {
    /* functions that are called by the view ... */
}

pub trait View<M: Model> {
    fn run(self);
    fn update(&mut self, model: &M); // could be made more specific.
    fn set_listener(&mut self, listener: Box<dyn ViewListener>);
}
