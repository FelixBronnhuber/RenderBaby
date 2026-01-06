use std::path::PathBuf;
use include_dir::File;
use crate::data_plane::scene::render_scene::Scene;
use crate::data_plane::scene_proxy::proxy_scene::ProxyScene;
use glam::Vec3;
use scene_objects::{camera::Resolution, material::Material, sphere::Sphere};

#[allow(dead_code)]
pub struct Model {
    pub scene: Scene,
    pub proxy: ProxyScene,
}

#[allow(dead_code)]
impl Model {
    pub fn new_from_path(path: PathBuf) -> anyhow::Result<Self> {
        match Scene::load_scene_from_file(path) {
            Ok(scene) => Ok(Self::new(scene)),
            Err(e) => Err(e),
        }
    }

    pub fn new_with_capsule() -> Self {
        let mut scene = Scene::new();
        // scene.proto_init();

        // Load capsule fixture
        let cwd = std::env::current_dir().unwrap();
        let capsule_path = cwd.join("fixtures/capsule/capsule.obj");
        if capsule_path.exists() {
            log::info!("Loading capsule fixture from {:?}", capsule_path);
            if let Err(e) = scene.load_object_from_file(capsule_path) {
                log::error!("Failed to load capsule fixture: {}", e);
            }
            // Set up camera for capsule
            scene
                .get_camera_mut()
                .set_position(Vec3::new(0.0, 2.0, 4.0));
            scene.get_camera_mut().set_look_at(Vec3::new(0.0, 0.0, 0.0));
            scene
                .get_camera_mut()
                .set_resolution(Resolution::new(256, 256));
        } else {
            log::warn!(
                "Capsule fixture not found at {:?}, falling back to proto_init",
                capsule_path
            );
            scene.proto_init();
        }

        // Load ferris fixture
        let ferris_path = cwd.join("fixtures/ferris_low_poly/rustacean-3d.obj");
        if ferris_path.exists() {
            log::info!("Loading ferris fixture from {:?}", ferris_path);
            // Move ferris to (0, 0, 4) to be visible in reflection
            if let Err(e) = scene.load_object_from_file_transformed(
                ferris_path,
                Vec3::new(3.5, -0.2, -1.0),
                Vec3::new(-90.0, 250.0, 0.0),
                1.0,
            ) {
                log::error!("Failed to load ferris fixture: {}", e);
            }
        } else {
            log::warn!("Ferris fixture not found at {:?}", ferris_path);
        }

        scene.add_sphere(Sphere::new(
            Vec3::new(2.0, 0.0, 2.0),
            1.0,
            Material::default(),
            [1.0, 1.0, 1.0],
        ));
        scene.set_color_hash_enabled(false); // Disable color hash to see textures

        Self::new(scene)
    }

    pub fn new_from_template(_file: &'static File<'static>) -> anyhow::Result<Self> {
        todo!()
    }

    pub fn new_empty() -> Self {
        Self::new(Scene::new())
    }

    pub fn new(scene: Scene) -> Self {
        let proxy = scene.get_proxy_scene();
        Self { scene, proxy }
    }

    pub fn set_output_path(_path: PathBuf) -> anyhow::Result<()> {
        // ask scene to change the output path. This would require the destination not to already exist
        todo!()
    }

    pub fn save() -> anyhow::Result<()> {
        // throws an error if an output path isn't set
        todo!()
    }

    pub fn render(&self) -> anyhow::Result<()> {
        todo!()
    }

    pub fn frame_buffer_ref(&self) -> &String {
        todo!()
    }

    pub fn reload_proxy(&mut self) {
        self.proxy = self.scene.get_proxy_scene();
    }
}
