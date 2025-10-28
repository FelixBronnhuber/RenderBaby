use crate::model::Model;
use crate::view::View;

pub trait Controller<M: Model, V: View<M>> {
    fn new(model: M, view: V) -> Self;
    fn update_cycle(&mut self);
}