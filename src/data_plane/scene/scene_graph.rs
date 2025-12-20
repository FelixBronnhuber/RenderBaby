use scene_objects::{camera::Camera, light_source::LightSource, mesh::Mesh, sphere::Sphere};
use serde::Serialize;
/// The scene graphs holds all elements of the scene
#[derive(Serialize)]
pub(crate) struct SceneGraph {
    spheres: Vec<Sphere>,
    #[serde(skip_serializing)]
    meshes: Vec<Mesh>,
    light_sources: Vec<LightSource>,
    camera: Camera,
}
#[allow(dead_code)]
impl SceneGraph {
    pub fn new() -> Self {
        //! ## Returns
        //! a new scene graph with emtpy objects, light_sources, and a default Camera
        Self {
            spheres: Vec::new(),
            meshes: Vec::new(),
            light_sources: Vec::new(),
            camera: Camera::default(),
        }
    }
    pub fn add_sphere(&mut self, sphere: Sphere) {
        //! Adds a Sphere
        //! ## Parameter
        //! 'sphere': render object to be added
        self.spheres.push(sphere);
    }
    pub fn add_mesh(&mut self, mesh: Mesh) {
        //! Adds a object
        //! ## Parameter
        //! 'mesh': render object to be added
        self.meshes.push(mesh);
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
        //! 'camera': camera to be stored
        self.camera = camera;
    }
    pub fn get_spheres(&self) -> &Vec<Sphere> {
        //! ## Returns
        //! all Spheres objects as a reference to a vector of Sphere
        &self.spheres
    }
    pub fn get_meshes(&self) -> &Vec<Mesh> {
        //! ## Returns
        //! all Meshes objects as a reference to a vector of Mesh
        &self.meshes
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

    pub fn remove_sphere(&mut self, index: usize) {
        //! Removes sphere at given index
        //! ## Parameter
        //! 'index': Index of the object that will be removed
        self.spheres.remove(index);
    }

    pub fn clear_spheres(&mut self) {
        self.spheres.clear();
    }

    pub fn clear_meshes(&mut self) {
        self.meshes.clear();
    }

    pub fn remove_mesh(&mut self, index: usize) {
        //! Removes mesh at given index
        //! ## Parameter
        //! 'index': Index of the object that will be removed
        self.meshes.remove(index);
    }

    pub fn remove_light_source(&mut self, index: usize) {
        //! Removes light source at given index
        //! ## Parameter
        //! 'index': Index of the LightSource that will be removed
        self.light_sources.remove(index);
    }
}
