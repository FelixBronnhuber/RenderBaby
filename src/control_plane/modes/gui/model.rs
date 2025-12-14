use crate::data_plane::scene::render_scene::Scene;

#[allow(dead_code)]
pub struct Model {
    scene: Scene,
}

#[allow(dead_code)]
impl Model {
    pub fn new(scene: Scene) -> Self {
        Self { scene }
    }
}
