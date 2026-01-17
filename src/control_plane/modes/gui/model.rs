use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use include_dir::File;
use crate::data_plane::scene::render_scene::Scene;
use crate::data_plane::scene_proxy::proxy_scene::ProxyScene;
use glam::Vec3;
use frame_buffer::frame_buffer::FrameBuffer;
use scene_objects::{camera::Resolution, material::Material, sphere::Sphere};

#[allow(dead_code)]
pub struct Model {
    pub scene: Arc<Mutex<Scene>>,
    pub proxy: ProxyScene,
    // flag to indicate whether the real scene has been modified without also modifying the proxy
    pub proxy_dirty: Arc<AtomicBool>,
    pub frame_buffer: FrameBuffer,
    pub export_misc: bool,
}

#[allow(dead_code)]
impl Model {
    pub fn new_from_path(path: PathBuf) -> anyhow::Result<Self> {
        match Scene::load_scene_from_path(path, false) {
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
            scene.get_camera_mut().set_ray_samples(1);
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

    pub fn new_from_template(file: &'static File<'static>) -> anyhow::Result<Self> {
        let file_contents = file
            .contents_utf8()
            .expect("Couldn't turn file into string.");
        Ok(Self::new(Scene::load_scene_from_string(
            file_contents.to_string(),
        )?))
    }

    pub fn new_empty() -> Self {
        Self::new(Scene::new())
    }

    pub fn new(scene: Scene) -> Self {
        let proxy = scene.get_proxy_scene();
        Self {
            scene: Arc::new(Mutex::new(scene)),
            proxy,
            proxy_dirty: Arc::new(AtomicBool::new(false)),
            frame_buffer: FrameBuffer::new(true),
            export_misc: Arc::new(AtomicBool::new(true)),
        }
    }

    pub fn set_output_path(&mut self, path: Option<PathBuf>) -> anyhow::Result<()> {
        // ask scene to change the output path. This would require the destination not to already exist
        self.scene.lock().unwrap().set_output_path(path)
    }

    pub fn render(&self) -> anyhow::Result<()> {
        self.frame_buffer
            .provide(self.scene.lock().unwrap().get_frame_iterator()?);
        Ok(())
    }

    pub fn reload_proxy(&mut self) {
        self.proxy = self.scene.lock().unwrap().get_proxy_scene();
    }

    pub fn mark_proxy_dirty(&self) {
        self.proxy_dirty.store(true, Ordering::SeqCst);
    }

    pub fn consume_proxy_dirty_and_reload(&mut self) {
        if self.proxy_dirty.swap(false, Ordering::SeqCst) {
            self.reload_proxy();
        }
    }
}
