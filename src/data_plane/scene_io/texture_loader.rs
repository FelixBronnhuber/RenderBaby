use std::collections::HashMap;
use std::path::Path;
use engine_config::TextureData;

#[derive(Default)]
pub struct TextureCache {
    map: HashMap<String, TextureData>,
}

impl TextureCache {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    /// Loads a texture from disk into the cache if not already present.
    /// Returns an opaque handle (index) for the texture. Currently this is just
    /// the insertion order index; existing items keep their index.
    pub fn load(&mut self, path: &str) -> anyhow::Result<i32> {
        let key = Path::new(path).to_string_lossy().to_string();

        if !self.map.contains_key(&key) {
            let img = image::open(&key)?;
            let img = img.to_rgba8();
            let (width, height) = img.dimensions();
            let data: Vec<u32> = img.pixels().map(|p| u32::from_le_bytes(p.0)).collect();
            self.map
                .insert(key.clone(), TextureData::new(width, height, data));
        }

        let mut keys: Vec<&String> = self.map.keys().collect();
        keys.sort();
        let idx = keys.iter().position(|k| **k == key).unwrap_or(0) as i32;
        Ok(idx)
    }
}
