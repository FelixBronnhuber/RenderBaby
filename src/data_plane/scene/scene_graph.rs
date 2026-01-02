use scene_objects::{camera::Camera, light_source::LightSource, mesh::Mesh, sphere::Sphere};
/// The scene graphs holds all elements of the scene
pub(crate) struct SceneGraph {
    spheres: Vec<Sphere>,
    meshes: Vec<Mesh>,
    light_sources: Vec<LightSource>,
    camera: Camera,
}
#[allow(dead_code)]
impl SceneGraph {
    pub(crate) fn new() -> Self {
        //! ## Returns
        //! a new scene graph with emtpy objects, light_sources, and a default Camera
        Self {
            spheres: Vec::new(),
            meshes: Vec::new(),
            light_sources: Vec::new(),
            camera: Camera::default(),
        }
    }
    pub(crate) fn add_sphere(&mut self, sphere: Sphere) {
        //! Adds a Sphere
        //! ## Parameter
        //! 'sphere': render object to be added
        self.spheres.push(sphere);
    }
    pub(crate) fn add_mesh(&mut self, mesh: Mesh) {
        //! Adds a object
        //! ## Parameter
        //! 'mesh': render object to be added
        self.meshes.push(mesh);
    }
    pub(crate) fn add_lightsource(&mut self, light: LightSource) {
        //! adds a LightSource
        //! ## Parameter
        //! 'light': LightSource to be added
        self.light_sources.push(light);
    }
    pub(crate) fn set_camera(&mut self, camera: Camera) {
        //! sets the camera
        //! ## Parameter
        //! 'camera': camera to be stored
        self.camera = camera;
    }
    pub(crate) fn get_spheres(&self) -> &Vec<Sphere> {
        //! ## Returns
        //! all Spheres objects as a reference to a vector of Sphere
        &self.spheres
    }
    pub(crate) fn get_spheres_mut(&mut self) -> &mut Vec<Sphere> {
        //! ## Returns
        //! all Spheres objects as a reference to a vector of Sphere
        &mut self.spheres
    }
    pub(crate) fn get_meshes(&self) -> &Vec<Mesh> {
        //! ## Returns
        //! all Meshes objects as a reference to a vector of Mesh
        &self.meshes
    }
    pub(crate) fn get_light_sources(&self) -> &Vec<LightSource> {
        //! ## Returns
        //! all light sources as a reference to a vector of LightSource
        &self.light_sources
    }
    pub(crate) fn get_camera_mut(&mut self) -> &mut Camera {
        //! ## Returns
        //! Mutable reference to the camera
        &mut self.camera
    }
    pub(crate) fn get_camera(&self) -> &Camera {
        //! ## Returns
        //! Reference to the camera
        &self.camera
    }

    pub(crate) fn remove_sphere(&mut self, index: usize) {
        //! Removes sphere at given index
        //! ## Parameter
        //! 'index': Index of the object that will be removed
        self.spheres.remove(index);
    }

    pub(crate) fn clear_spheres(&mut self) {
        self.spheres.clear();
    }

    pub(crate) fn clear_meshes(&mut self) {
        self.meshes.clear();
    }

    pub(crate) fn remove_mesh(&mut self, index: usize) {
        //! Removes mesh at given index
        //! ## Parameter
        //! 'index': Index of the object that will be removed
        self.meshes.remove(index);
    }

    pub(crate) fn remove_light_source(&mut self, index: usize) {
        //! Removes light source at given index
        //! ## Parameter
        //! 'index': Index of the LightSource that will be removed
        self.light_sources.remove(index);
    }
}
