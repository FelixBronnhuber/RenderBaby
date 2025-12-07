pub mod egui_view;

pub type EventResult = anyhow::Result<Box<dyn std::any::Any>>;
pub type EventHandler<E> = dyn FnMut(E) -> EventResult + Send + 'static;

pub trait ViewWrapper<E, P>: Sized + 'static {
    fn new(pipeline: P, handler: Box<EventHandler<E>>) -> Self;
    fn open(self);
}
