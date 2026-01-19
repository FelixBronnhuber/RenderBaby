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
    /// ## Returns
    /// a new scene graph with emtpy objects, light_sources, and a default Camera
    pub fn new() -> Self {
        Self {
            spheres: Vec::new(),
            meshes: Vec::new(),
            light_sources: Vec::new(),
            camera: Camera::default(),
        }
    }
    /// Adds a Sphere
    /// ## Parameter
    /// 'sphere': render object to be added
    pub fn add_sphere(&mut self, sphere: Sphere) {
        self.spheres.push(sphere);
    }
    /// Adds a object
    /// ## Parameter
    /// 'mesh': render object to be added
    pub fn add_mesh(&mut self, mesh: Mesh) {
        self.meshes.push(mesh);
    }
    /// adds a LightSource
    /// ## Parameter
    /// 'light': LightSource to be added
    pub fn add_lightsource(&mut self, light: LightSource) {
        self.light_sources.push(light);
    }
    /// sets the camera
    /// ## Parameter
    /// 'camera': camera to be stored
    pub fn set_camera(&mut self, camera: Camera) {
        self.camera = camera;
    }
    /// ## Returns
    /// all Spheres objects as a reference to a vector of Sphere
    pub fn get_spheres(&self) -> &Vec<Sphere> {
        &self.spheres
    }
    /// ## Returns
    /// all Spheres objects as a reference to a vector of Sphere
    pub fn get_spheres_mut(&mut self) -> &mut Vec<Sphere> {
        &mut self.spheres
    }
    /// ## Returns
    /// all Meshes objects as a reference to a vector of Mesh
    pub fn get_meshes(&self) -> &Vec<Mesh> {
        &self.meshes
    }
    /// ## Returns
    /// all Meshes objects as a reference to a vector of Mesh
    pub fn get_meshes_mut(&mut self) -> &mut Vec<Mesh> {
        &mut self.meshes
    }
    /// ## Returns
    /// all light sources as a reference to a vector of LightSource
    pub fn get_light_sources(&self) -> &Vec<LightSource> {
        &self.light_sources
    }
    /// ## Returns
    /// all light sources as a reference to a vector of LightSource
    pub fn get_light_sources_mut(&mut self) -> &mut Vec<LightSource> {
        &mut self.light_sources
    }
    /// ## Returns
    /// Mutable reference to the camera
    pub fn get_camera_mut(&mut self) -> &mut Camera {
        &mut self.camera
    }
    /// ## Returns
    /// Reference to the camera
    pub fn get_camera(&self) -> &Camera {
        &self.camera
    }
    /// Removes sphere at given index
    /// ## Parameter
    /// 'index': Index of the object that will be removed
    pub fn remove_sphere(&mut self, index: usize) {
        self.spheres.remove(index);
    }
    /// Deletes all spheres in the graph
    pub fn clear_spheres(&mut self) {
        self.spheres.clear();
    }
    /// Deletes all meshes in the graph
    pub fn clear_meshes(&mut self) {
        self.meshes.clear();
    }
    /// Removes mesh at given index
    /// ## Parameter
    /// 'index': Index of the object that will be removed
    pub fn remove_mesh(&mut self, index: usize) {
        self.meshes.remove(index);
    }
    /// Removes light source at given index
    /// ## Parameter
    /// 'index': Index of the LightSource that will be removed
    pub fn remove_light_source(&mut self, index: usize) {
        self.light_sources.remove(index);
    }
}
