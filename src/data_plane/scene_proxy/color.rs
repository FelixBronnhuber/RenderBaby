use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub(crate) struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Color {
    pub(crate) fn new_from_array(c: [f32; 3]) -> Self {
        Self {
            r: c[0],
            g: c[1],
            b: c[2],
        }
    }
}
impl PartialEq<[f32; 3]> for Color {
    fn eq(&self, other: &[f32; 3]) -> bool {
        self.r == other[0] && self.g == other[1] && self.b == other[2]
    }
}

/* impl From<[f32;3]> for Color {
    fn from(value: [f32;3]) -> Self {
        Self {
            r: value[0],
            g: value[1],
            b: value[2]
        }
    }
} */

impl From<Color> for [f32; 3] {
    fn from(value: Color) -> Self {
        [value.r, value.g, value.b]
    }
}
