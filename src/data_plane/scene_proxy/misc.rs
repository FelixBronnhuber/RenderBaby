use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Misc {
    pub ray_samples: u32,
    pub color_hash_enabled: bool,
}
