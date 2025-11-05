use crate::mvc::Model;

pub struct FullModel {

}

impl FullModel {
    pub fn new() -> Self {
        Self {}
    }
}

impl Model for FullModel {
    fn update(&mut self) {
        todo!()
    }
}
