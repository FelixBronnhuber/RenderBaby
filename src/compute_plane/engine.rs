use anyhow::Result;
use engine_config::{Renderer, RenderConfig};
use engine_config::renderer::RendererIterable;
use frame_buffer::frame_iterator::{Frame, FrameIterator};
use std::time::Instant;
use chrono::Local;

use crate::compute_plane::render_engine::RenderEngine;

pub struct Engine {
    renderer: Box<dyn Renderer + Sync>,
    engine_type: RenderEngine,
}

impl RendererIterable for Engine {
    fn render(&mut self, rc: RenderConfig) -> Result<Frame> {
        log::info!("Render started at {}", Local::now());
        let start = Instant::now();

        let mut frame_iterator = self.get_frame_iterator(rc)?;
        let mut last_frame: Frame = Frame::new(0, 0, vec![]);
        loop {
            if frame_iterator.has_next() {
                last_frame = frame_iterator.next().unwrap();
            } else {
                break;
            }
        }

        let duration = start.elapsed();
        log::info!("Render finished in {:?}", duration);
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

    pub fn render(&mut self, rc: RenderConfig) -> Result<Frame> {
        log::info!("Render started at {}", Local::now());
        let start = Instant::now();

        let result = self.renderer.render(rc);

        let duration = start.elapsed();
        log::info!("Render finished in {:?}", duration);

        result
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
