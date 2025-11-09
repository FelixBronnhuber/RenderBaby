use crate::mvc;

pub struct Model {

}

impl Model {
    pub fn new() -> Self {
        Self {}
    }
}

impl mvc::Model for Model {
    fn update(&mut self) {
        todo!()
    }
}
