use glam::Vec3;
use scene_objects::{camera::Camera, geometric_object::GeometricObject, light_source::LightSource};

pub(crate) struct SceneGraph {
    objects: Vec<Box<dyn GeometricObject>>,
    light_sources: Vec<LightSource>,
    camera: Camera,
}
#[allow(dead_code)]
impl SceneGraph {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
            light_sources: Vec::new(),
            camera: Camera::new(Vec3::new(0.0, 0.0, 0.0), Vec3::default()),
        }
    }
    pub fn add_object(&mut self, obj: Box<dyn GeometricObject>) {
        self.objects.push(obj);
    }
    pub fn add_lightsource(&mut self, light: LightSource) {
        self.light_sources.push(light);
    }
    pub fn set_camera(&mut self, camera: Camera) {
        self.camera = camera;
    }
    pub fn get_objects(&self) -> &Vec<Box<dyn GeometricObject>> {
        &self.objects
    }
    pub fn get_light_sources(&self) -> &Vec<LightSource> {
        &self.light_sources
    }
    pub fn get_camera_mut(&mut self) -> &mut Camera {
        &mut self.camera
    }
    pub fn get_camera(&self) -> &Camera {
        &self.camera
    }

    pub fn remove_object(&mut self, index: usize) {
        self.objects.remove(index);
    }

    pub fn remove_light_source(&mut self, index: usize) {
        self.light_sources.remove(index);
    }
}
