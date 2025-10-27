use crate::model::Model;
use crate::view::{View, ViewListener};

pub trait Controller<M: Model, V: View<M>>: ViewListener {
    fn new(model: M, view: V) -> Self;
    fn update_cycle(&mut self);
}