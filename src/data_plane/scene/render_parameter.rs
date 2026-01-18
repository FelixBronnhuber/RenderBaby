use engine_config::Uniforms;
use serde::{Deserialize, Serialize};
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub(crate) struct RenderParameter {
    pub(crate) ground_height: f32,
    pub(crate) ground_enabled: bool,
    pub(crate) checkerboard_enabled: bool,
    pub(crate) checkerboard_colors: ([f32; 3], [f32; 3]),
    pub(crate) max_depth: u32,
    pub(crate) sky_color: [f32; 3],
    pub(crate) color_hash_enabled: bool,
}

impl Default for RenderParameter {
    fn default() -> Self {
        let uniform = Uniforms::default();
        Self {
            ground_height: uniform.ground_height,
            ground_enabled: Uniforms::GROUND_ENABLED,
            checkerboard_enabled: Uniforms::CHECKERBOARD_ENABLED,
            checkerboard_colors: (uniform.checkerboard_color_1, uniform.checkerboard_color_2),
            max_depth: uniform.max_depth,
            sky_color: uniform.sky_color,
            color_hash_enabled: true,
        }
    }
}
