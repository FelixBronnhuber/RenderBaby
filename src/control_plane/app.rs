pub trait App {
    fn new() -> Self
    where
        Self: Sized;
    fn show(self: Box<Self>);
}
