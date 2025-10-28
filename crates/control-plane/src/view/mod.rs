use crate::model::Model;


pub trait View<M: Model> {
    fn run(&mut self, listener: &ViewListener);
    fn update(&mut self, model: &M);
}
