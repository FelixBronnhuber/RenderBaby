pub mod egui_view;

pub trait ViewWrapper<E, P>: Sized + 'static {
    fn new(pipeline: P, handler: Box<dyn FnMut(E)>) -> Self;
    fn open(self);
}
