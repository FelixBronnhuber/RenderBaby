use anyhow::Result;
use engine_config::{RenderOutput, Renderer, RenderConfig};
use engine_config::renderer::RendererIterable;
use frame_buffer::frame_iterator::{Frame, FrameIterator};

use crate::compute_plane::render_engine::RenderEngine;

pub struct Engine {
    renderer: Box<dyn Renderer + Sync>,
    engine_type: RenderEngine,
}

impl RendererIterable for Engine {
    fn render(&mut self, rc: RenderConfig) -> Result<Frame> {
        let mut frame_iterator = self.get_frame_iterator(rc)?;
        let mut last_frame: Frame = Frame::new(0, 0, vec![]);
        loop {
            if frame_iterator.has_next() {
                last_frame = frame_iterator.next().unwrap();
            } else {
                break;
            }
        }
        Ok(last_frame)
    }

    fn get_frame_iterator(&mut self, rc: RenderConfig) -> Result<Box<dyn FrameIterator>> {
        self.renderer.frame_iterator(rc)
    }
}

impl Engine {
    pub fn new(rc: RenderConfig, engine_type: RenderEngine) -> Self {
        let renderer: Box<dyn Renderer + Sync> = match engine_type {
            RenderEngine::Raytracer => Box::new(engine_raytracer::Engine::new(rc)),
            RenderEngine::Pathtracer => Box::new(engine_pathtracer::Engine::new(rc)),
        };

        Self {
            renderer,
            engine_type,
        }
    }

    pub fn render(&mut self, rc: RenderConfig) -> Result<RenderOutput> {
        self.renderer.render(rc)
    }

    #[allow(dead_code)]
    pub fn switch_engine(&mut self, rc: RenderConfig, engine_type: RenderEngine) {
        self.renderer = match engine_type {
            RenderEngine::Raytracer => Box::new(engine_raytracer::Engine::new(rc)),
            RenderEngine::Pathtracer => Box::new(engine_pathtracer::Engine::new(rc)),
        };
        self.engine_type = engine_type;
    }

    #[allow(dead_code)]
    pub fn current_engine(&self) -> RenderEngine {
        self.engine_type
    }
}
