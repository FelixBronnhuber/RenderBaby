use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub(crate) struct Color {
    r: f32,
    g: f32,
    b: f32,
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
