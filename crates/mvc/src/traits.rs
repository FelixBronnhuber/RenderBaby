pub trait ViewListener {
    type Event: Clone;

    fn handle_event(&mut self, event: Self::Event);
}

pub trait View: Sized {
    type Pipeline;
    type Event: Clone;

    fn new(pipeline: Self::Pipeline) -> Self;
    fn set_listener(&mut self, listener: Box<dyn ViewListener<Event = Self::Event>>);
    fn open(self);
}

pub trait Controller: ViewListener {
    type Pipeline;
    type Model;

    fn new(pipeline: Self::Pipeline, model: Self::Model) -> Self;
}
