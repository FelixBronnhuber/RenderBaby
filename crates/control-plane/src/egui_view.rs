use crate::mvc::View;
use crate::controller::FullViewListener;
use crate::model::FullModel;

pub struct EGuiView {
    listener: Option<Box<dyn FullViewListener>>,
}

impl EGuiView {
    pub fn new() -> Self {
        Self { listener: None }
    }
}

impl<V> View<FullModel, V> for EGuiView
    where V: FullViewListener + 'static
{
    fn run(self) {
        todo!()
    }

    fn update(&mut self, model: &FullModel) {
        todo!()
    }

    fn set_listener(&mut self, listener: Box<V>) {
        self.listener = Some(listener);
    }
}
