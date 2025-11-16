use anyhow::Result;
use engine_config::{RenderConfig, RenderEngine};
use engine_core::{RenderOutput, Renderer};

pub struct Engine {
    renderer: Box<dyn Renderer>,
    engine_type: RenderEngine,
}

impl Engine {
    pub fn new(rc: RenderConfig) -> Self {
        let engine_type = rc.engine;

        let renderer: Box<dyn Renderer> = match engine_type {
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

    pub fn switch_engine(&mut self, rc: RenderConfig, engine_type: RenderEngine) {
        self.renderer = match engine_type {
            RenderEngine::Raytracer => Box::new(engine_raytracer::Engine::new(rc)),
            RenderEngine::Pathtracer => Box::new(engine_pathtracer::Engine::new(rc)),
        };
        self.engine_type = engine_type;
    }

    pub fn current_engine(&self) -> RenderEngine {
        self.engine_type
    }
}
