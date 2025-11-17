use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable, Debug, PartialEq)]
pub struct Vec3(pub [f32; 3]);

impl Vec3 {
    pub const ZERO: Vec3 = Vec3([0.0, 0.0, 0.0]);
    pub const ONE: Vec3 = Vec3([1.0, 1.0, 1.0]);

    pub const COLOR_RED: Vec3 = Vec3([1.0, 0.0, 0.0]);
    pub const COLOR_GREEN: Vec3 = Vec3([0.0, 1.0, 0.0]);
    pub const COLOR_BLUE: Vec3 = Vec3([0.0, 0.0, 1.0]);
    pub const COLOR_YELLOW: Vec3 = Vec3([1.0, 1.0, 0.0]);
    pub const COLOR_CYAN: Vec3 = Vec3([0.0, 1.0, 1.0]);
    pub const COLOR_MAGENTA: Vec3 = Vec3([1.0, 0.0, 1.0]);
    pub const COLOR_WHITE: Vec3 = Vec3([1.0, 1.0, 1.0]);
    pub const COLOR_BLACK: Vec3 = Vec3([0.0, 0.0, 0.0]);

    #[inline]
    pub fn x(&self) -> f32 {
        self.0[0]
    }

    #[inline]
    pub fn y(&self) -> f32 {
        self.0[1]
    }

    #[inline]
    pub fn z(&self) -> f32 {
        self.0[2]
    }

    pub fn is_valid_color(&self) -> bool {
        self.0.iter().all(|&c| c >= 0.0 && c <= 1.0)
    }

    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vec3([x, y, z])
    }

    pub fn scale(&self, lambda: f32) -> Vec3 {
        Vec3([self.0[0] * lambda, self.0[1] * lambda, self.0[2] * lambda])
    }
}
