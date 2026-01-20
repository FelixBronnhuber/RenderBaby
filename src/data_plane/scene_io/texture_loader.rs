use std::collections::HashMap;
use image::ImageFormat;
use engine_config::TextureData;
use crate::included_files::AutoPath;

#[derive(Default)]
pub struct TextureCache {
    map: HashMap<AutoPath<'static>, TextureData>,
}

impl TextureCache {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn get_split_clone(&self) -> (Vec<TextureData>, HashMap<String, i32>) {
        let mut textures: Vec<TextureData> = Vec::new();
        let mut map: HashMap<String, i32> = HashMap::new();
        for (key, val) in &self.map {
            textures.push(val.clone());
            map.insert(key.to_string(), textures.len() as i32 - 1);
        }
        (textures, map)
    }

    pub fn load(&mut self, path: AutoPath) -> anyhow::Result<i32> {
        // Normalize key to a 'static AutoPath using its owned path buffer
        let key_path = path.path_buf();
        let key_auto: AutoPath<'static> = AutoPath::try_from(key_path)?;

        if !self.map.contains_key(&key_auto) {
            let img = image::load(
                path.reader()?,
                ImageFormat::from_extension(path.extension().unwrap()).unwrap_or(ImageFormat::Png),
            )?;
            let img = img.to_rgba8();
            let (width, height) = img.dimensions();
            let data: Vec<u32> = img.pixels().map(|p| u32::from_le_bytes(p.0)).collect();
            self.map
                .insert(key_auto.clone(), TextureData::new(width, height, data));
        }

        // Determine index based on sorted key display strings for determinism
        let mut keys: Vec<String> = self.map.keys().map(|k| k.to_string()).collect();
        keys.sort();
        let search_key = key_auto.to_string();
        let idx = keys.iter().position(|k| *k == search_key).unwrap_or(0) as i32;
        Ok(idx)
    }
}
