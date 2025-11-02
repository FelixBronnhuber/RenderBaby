use crate::geometric_object::{Camera, GeometricObject, LightSource};

pub struct SceneGraph {
    objects: Vec<GeometricObject>,
    light_sources: Vec<LightSource>,
    camera: Camera
}

impl SceneGraph {
    // todo
    pub fn new() {}
    pub fn add_object(&mut self, obj: GeometricObject) {
    }
    pub fn add_lightsource(&mut self, light: LightSource) {}
    pub fn set_camera(&mut self, camera: Camera) {
        self.camera = camera;
    }
    // todo get etc ...
}