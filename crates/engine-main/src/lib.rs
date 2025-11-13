use std::error::Error;
use engine_raytracer::{RenderConfig, RenderOutput, RenderState};
use anyhow::{Result, };

pub struct Engine {
    pub renderer: RenderState
}

impl Engine {
    pub fn new(rc: RenderConfig) -> Engine {
        Self{
            renderer: RenderState::new(rc)
        }
    }

    pub fn render(&mut self, rc: RenderConfig) -> Result<RenderOutput> {
        self.renderer.render(rc)
    }
}