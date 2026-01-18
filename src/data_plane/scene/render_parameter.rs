use engine_config::Uniforms;
#[derive(Copy, Clone, Debug)]
pub(super) struct RenderParameter {
    pub(super) ground_height: f32,
    pub(super) ground_enabled: bool,
    pub(super) checkerboard_enabled: bool,
    pub(super) checkerboard_colors: ([f32; 3], [f32; 3]),
    pub(super) max_depth: u32,
    pub(super) sky_color: [f32; 3],
    pub(super) color_hash_enabled: bool,
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
