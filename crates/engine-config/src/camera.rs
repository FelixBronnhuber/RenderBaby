use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct Camera {
    pub pane_distance: f32,
    pub pane_width: f32,
    _pad0: [f32; 2],
    pub pos: [f32; 3],
    _pad1: f32,
    pub dir: [f32; 3],
    _pad2: f32,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            pane_distance: 50.0,
            pane_width: 100.0,
            _pad0: [0.0; 2],
            pos: [2.0, 2.0, 0.0],
            _pad1: 0.0,
            dir: [0.0, 0.0, 1.0],
            _pad2: 0.0,
        }
    }
}

impl Camera {
    pub fn new(pane_distance: f32, pane_width: f32, pos: [f32; 3], dir: [f32; 3]) -> Self {
        Self {
            pane_distance,
            pane_width,
            pos,
            dir,
            ..Default::default()
        }
    }
}
