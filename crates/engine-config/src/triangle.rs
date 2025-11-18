use anyhow::Result;
use bytemuck::{Pod, Zeroable};

use crate::Vec3;

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable, Debug, PartialEq)]
pub struct Triangle(pub [u32; 4]);

impl Triangle {
    pub fn new(v0: u32, v1: u32, v2: u32) -> Self {
        Self([v0, v1, v2, 0])
    }

    pub fn v0(self) -> u32 {
        self.0[0]
    }

    pub fn v1(self) -> u32 {
        self.0[1]
    }

    pub fn v2(self) -> u32 {
        self.0[2]
    }

    // TODO: Introduce newtype for verticies?
    pub fn validate_against_verticies(&self, verticies: Vec<Vec3>) -> Result<Self> {
        let _ = verticies;
        todo!("Implement validation of triangle indicies against verticies");
    }
}
