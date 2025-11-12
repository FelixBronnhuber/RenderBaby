use crate::view::{Event, ViewListener};
use crate::model::Model;
use engine_raytracer::*;
use engine_wgpu_wrapper::{WgpuWrapper, EngineType};
use crossbeam::channel::Sender;

pub struct Controller {
    pub model: Model,
    pub tx: Sender<RenderOutput>,
    pub renderer: WgpuWrapper,
}

impl Controller {
    pub fn new(model: Model, tx: Sender<RenderOutput>, width: usize, height: usize) -> Self {
        let renderer = WgpuWrapper::new(EngineType::Raytracer, width, height)
            .expect("Renderer initialization failed");

        Self { model, tx, renderer }
    }
}

impl ViewListener for Controller {
    fn handle_event(&mut self, _event: Event) {

        match self.renderer.render() {
            Ok(mut output) => {
                if output.validate().is_ok() {
                    let _ = self.tx.send(output);
                } else {
                    eprintln!("Invalid RenderOutput!");
                }
            }
            Err(e) => eprintln!("Render failed: {:?}", e),
        }
    }
}
