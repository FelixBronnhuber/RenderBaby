use scene_objects::{
    camera::Camera, light_source::LightSource, mesh::Mesh, sphere::Sphere,
    tri_geometry::TriGeometry,
};
/// The scene graphs holds all elements of the scene
pub(crate) struct SceneGraph {
    tri_geometries: Vec<TriGeometry>,
    spheres: Vec<Sphere>,
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
            tri_geometries: Vec::new(),
            spheres: Vec::new(),
            meshes: Vec::new(),
            light_sources: Vec::new(),
            camera: Camera::default(),
        }
    }
    pub fn add_tri_geometry(&mut self, tri: TriGeometry) {
        //! Adds a TriGeometry
        //! ## Parameter
        //! 'tri': render object to be added
        self.tri_geometries.push(tri);
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

    pub fn get_tri_geometries(&self) -> &Vec<TriGeometry> {
        //! ## Returns
        //! all TriGeometries as a reference to a vector of TriGeometry
        &self.tri_geometries
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

    pub fn clear_tri_geometries(&mut self) {
        self.tri_geometries.clear();
    }

    pub fn remove_tri_geomtry(&mut self, index: usize) {
        //! Removes TriGeometry at given index
        //! ## Parameter
        //! 'index': Index of the object that will be removed
        self.tri_geometries.remove(index);
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
