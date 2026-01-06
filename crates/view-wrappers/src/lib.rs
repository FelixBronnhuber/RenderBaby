pub mod egui_view;

pub trait ViewWrapper: Sized + 'static {
    fn new() -> Self;
    fn open(self);
}
