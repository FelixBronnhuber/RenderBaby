use scene_objects::{camera::Camera, geometric_object::GeometricObject, light_source::LightSource};
/// The scene graphs holds all elements of the scene
pub(crate) struct SceneGraph {
    objects: Vec<Box<dyn GeometricObject>>,
    light_sources: Vec<LightSource>,
    camera: Camera,
}
#[allow(dead_code)]
impl SceneGraph {
    pub fn new() -> Self {
        //! ## Returns
        //! a new scene graph with emtpy objects, light_sources, and a default Camera
        Self {
            objects: Vec::new(),
            light_sources: Vec::new(),
            camera: Camera::default(),
        }
    }
    pub fn add_object(&mut self, obj: Box<dyn GeometricObject>) {
        //! Adds a object
        //! ## Parameter
        //! 'obj': render object to be added
        self.objects.push(obj);
    }
    pub fn add_lightsource(&mut self, light: LightSource) {
        //! adds a LightSource
        //! ## Parameter
        //! 'light': LightSource to be added
        self.light_sources.push(light);
    }
    pub fn set_camera(&mut self, camera: Camera) {
        //! sets the camera
        //! ## Parameter
        //! 'camer': camera to be stored
        self.camera = camera;
    }
    pub fn get_objects(&self) -> &Vec<Box<dyn GeometricObject>> {
        //! ## Returns
        //! all render objects as a reference to a vector of GeometricObject
        &self.objects
    }
    pub fn get_light_sources(&self) -> &Vec<LightSource> {
        //! ## Returns
        //! all light sources as a reference to a vector of LightSource
        &self.light_sources
    }
    pub fn get_camera_mut(&mut self) -> &mut Camera {
        //! ## Returns
        //! Mutable reference to the camera
        &mut self.camera
    }
    pub fn get_camera(&self) -> &Camera {
        //! ## Returns
        //! Reference to the camera
        &self.camera
    }

    pub fn remove_object(&mut self, index: usize) {
        //! Removes object at given index
        //! ## Parameter
        //! 'index': Index of the object that will be removed
        self.objects.remove(index);
    }

    pub fn remove_light_source(&mut self, index: usize) {
        //! Removes light source at given index
        //! ## Parameter
        //! 'index': Index of the LightSource that will be removed
        self.light_sources.remove(index);
    }
}
