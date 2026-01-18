//! Simple wrappers around views that can be opened
pub mod egui_view;

/// Trait for wrappers around views that can be opened
pub trait ViewWrapper: Sized + 'static {
    /// Creates a new instance of the [`ViewWrapper`]. Does not take any arguments.
    fn new() -> Self;

    /// Opens the view.
    fn open(self);
}
