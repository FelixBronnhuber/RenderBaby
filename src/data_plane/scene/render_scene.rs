use std::path::PathBuf;
use anyhow::Error;
use engine_config::{RenderConfigBuilder, Uniforms};
use glam::Vec3;
use log::{debug, error, info, warn};
use frame_buffer::frame_iterator::Frame;
use scene_objects::{
    camera::{Camera, Resolution},
    geometric_object::GeometricObject,
    light_source::{LightSource},
    material::Material,
    mesh::Mesh,
    sphere::Sphere,
};
use crate::{
    compute_plane::{engine::Engine, render_engine::RenderEngine},
    data_plane::{
        scene::{render_parameter::RenderParameter, scene_graph::SceneGraph},
        scene_io::{img_export::export_img_png, obj_parser::load_obj, scene_importer::parse_scene},
    },
    included_files::AutoPath,
};
use crate::data_plane::scene_io::scene_exporter;
use crate::data_plane::scene_io::texture_loader::TextureCache;
use crate::data_plane::scene_proxy::color::Color;

/// The scene holds all relevant objects, lightsources, camera
pub struct Scene {
    scene_graph: SceneGraph,
    name: String,
    first_render: bool,
    render_engine: Option<Engine>,
    last_frame: Option<Frame>,
    pub(crate) texture_cache: TextureCache,
    output_path: Option<PathBuf>,
    render_params: RenderParameter,
}
impl Default for Scene {
    fn default() -> Self {
        Self::new()
    }
}

#[allow(unused)]
impl Scene {
    /// loads and returns a new scene from a json / rscn file at path
    fn _load_scene_from_path(auto_path: AutoPath) -> anyhow::Result<Scene> {
        info!("Scene: Loading new scene from {}", auto_path);

        let loaded_data = parse_scene(auto_path.clone(), None)?;

        let mut scene = loaded_data.scene;
        let paths = loaded_data.paths;
        let rotation = loaded_data.rotations;
        let translation = loaded_data.translations;
        let scale = loaded_data.scales;

        debug!("Scene: Loading {} objects...", paths.len());
        for (i, p_str) in paths.iter().enumerate() {
            let p = AutoPath::try_from(p_str.to_string())?;
            debug!("Scene: Loading object {} from {:?}", i, p);
            info!(
                "Scene: Applying transformations: translation={:?}, rotation={:?}, scale={:?}",
                translation[i], rotation[i], scale[i]
            );
            scene.load_object_from_file_relative(
                p.clone(),
                p.path_buf(),
                translation[i],
                rotation[i],
                scale[i],
            )?;
        }

        info!("Scene: Successfully loaded scene.");
        Ok(scene)
    }
    /// Export the scene to the given path
    /// ## Parameter:
    /// 'path' std::path::PathBuf to the scene (.rscn file)
    /// 'export_misc': If project specific parameters that go beyond the rscn-standard should be exported aswell
    pub fn export_scene(&self, path: PathBuf, export_misc: bool) -> Result<(), Error> {
        info!("{self}: Exporting scene");
        let result = scene_exporter::serialize_scene(path.clone(), self, export_misc);
        match result {
            Err(error) => {
                error!(
                    "{self}: exporting scene to {:?} resulted in error: {error}",
                    path
                );
                Err(error)
            }
            _ => {
                info!("{self}: Successfully exported scene to {}", path.display());
                Ok(())
            }
        }
    }

    // LOAD OBJECTS
    /// Loads the object at the given path and returns it as a Mesh
    /// ## Parameters
    /// 'path': std::path::PathBuf
    /// 'relative_path': Option<Path>. If not none used as mesh path
    pub fn parse_obj_to_mesh(
        &mut self,
        auto_path: AutoPath,
        relative_path: Option<PathBuf>,
    ) -> Result<Mesh, Error> {
        info!("{self}: Loading object from {}", auto_path);
        match load_obj(auto_path.clone(), &mut self.texture_cache) {
            Ok(mut res) => {
                // If a relative path is provided, override mesh path to keep exported scene clean
                if let Some(rel) = relative_path {
                    res.mesh.set_path(rel);
                }

                info!(
                    "Scene {self}: Successfully loaded object from {}",
                    auto_path
                );
                Ok(res.mesh)
            }
            Err(error) => {
                error!(
                    "{self}: Parsing obj from {} resulted in error: {error}",
                    auto_path
                );
                Err(error)
            }
        }
    }
    /// Loads a mesh from the given path and adds it to the meshes of the scene
    /// ## Parameter
    /// 'auto_path': AutoPath to the obj
    pub fn load_object_from_file(&mut self, auto_path: AutoPath) -> Result<(), Error> {
        let mesh = self.parse_obj_to_mesh(auto_path, None)?;
        self.add_mesh(mesh);
        Ok(())
    }
    /// Loads a mesh from the given  relative path and applies the given translation, rotation, scale, adds it to the scene
    /// 'auto_path':  relative AutoPath to the obj
    /// 'translation': Translation to be applied, as glam::Vec3
    /// 'rotation': Rotation to be applied, as glam::Vec3
    /// 'scale': Scale to be applied, as glam::Vec3
    fn load_object_from_file_relative(
        &mut self,
        auto_path: AutoPath,
        relative_path: PathBuf,
        translation: Vec3,
        rotation: Vec3,
        scale: Vec3,
    ) -> Result<(), Error> {
        let mut mesh = self.parse_obj_to_mesh(auto_path, Some(relative_path))?;
        mesh.scale(scale.x);
        mesh.rotate(rotation);
        mesh.translate(translation);
        self.add_mesh(mesh);
        Ok(())
    }
    /// Loads a mesh from the given  relative path and applies the given translation, rotation, scale, adds it to the scene
    /// 'path': absolute AutoPath to the obj
    /// 'translation': Translation to be applied, as glam::Vec3
    /// 'rotation': Rotation to be applied, as glam::Vec3
    /// 'scale': Scale to be applied, as glam::Vec3
    pub fn load_object_from_file_transformed(
        &mut self,
        path: AutoPath,
        translation: Vec3,
        rotation: Vec3,
        scale: f32,
    ) -> Result<(), Error> {
        let mut mesh = self.parse_obj_to_mesh(path, None)?;
        mesh.scale(scale);
        mesh.rotate(rotation);
        mesh.translate(translation);
        self.add_mesh(mesh);
        Ok(())
    }

    // LOAD SCENES
    /// Loads a scene from the given path pointing to a .rscn or .json-file
    /// # Parameter
    /// 'path': AutoPath to the scene file
    /// 'detached':
    pub fn load_scene_from_path(auto_path: AutoPath, detached: bool) -> anyhow::Result<Scene> {
        let mut scene = Self::_load_scene_from_path(auto_path.clone());
        match scene {
            Ok(mut scene) => {
                let is_rscn = match auto_path.extension() {
                    Some(ext) => ext == "rscn",
                    None => false,
                };

                if !is_rscn && !detached {
                    scene.output_path = Some(auto_path.path_buf());
                } else {
                    scene.output_path = None;
                }

                // TODO: Why is this always enabled in the first place?
                if is_rscn {
                    scene.set_color_hash_enabled(false);
                }

                Ok(scene)
            }
            Err(error) => Err(error),
        }
    }
    /// Saves the scene
    /// ## Parameter
    /// 'export_misc': if misc parameters should be exported aswell. These go beyond the rscn standard
    pub fn save(&mut self, export_misc: bool) -> anyhow::Result<()> {
        if let Some(output_path) = self.output_path.clone()
        // && output_path.exists() TODO: why would the path already need to exist?
        {
            self.export_scene(output_path, export_misc)?;
            Ok(())
        } else {
            Err(Error::msg("No valid output path set for this scene"))
        }
    }
    /// Sets the output path to the given path
    /// ## Parameter
    /// 'path': Option<std::path::PathBuf> new output_path of the scene
    pub fn set_output_path(&mut self, path: Option<PathBuf>) -> anyhow::Result<()> {
        self.output_path = path;
        Ok(())
    }
    /// Sets up a basic scene with some spheres for testing or as a fallback
    pub fn proto_init(&mut self) {
        info!("{self}: Initialising with 'proto' settings");
        let green = [0.0, 1.0, 0.0];
        let magenta = [1.0, 0.0, 1.0];
        let red = [1.0, 0.0, 0.0];
        let blue = [0.0, 0.0, 1.0];
        let cyan = [0.0, 1.0, 1.0];

        let sphere0 = Sphere::new(Vec3::new(0.0, 0.6, 2.0), 0.5, Material::default(), magenta);
        let sphere1 = Sphere::new(Vec3::new(-0.6, 0.0, 2.0), 0.5, Material::default(), green);
        let sphere2 = Sphere::new(Vec3::new(0.0, 0.0, 2.0), 0.5, Material::default(), red);
        let sphere3 = Sphere::new(Vec3::new(0.6, 0.0, 2.0), 0.5, Material::default(), blue);
        let sphere4 = Sphere::new(Vec3::new(0.0, -0.6, 2.0), 0.5, Material::default(), cyan);

        let cam = Camera::default();
        let ambient_light = LightSource::new(
            Vec3::new(0.0, 0.0, 3.0),
            0.0,
            [1.0, 1.0, 1.0],
            "proto_light".to_owned(),
            Vec3::default(),
            //LightType::Ambient,
        );
        let point_light = LightSource::new(
            Vec3::new(2.0, 4.0, 1.0),
            20.0,
            [1.0, 0.9, 0.8],
            "point_light".to_owned(),
            Vec3::default(),
            //LightType::Point,
        );
        self.add_sphere(sphere0);
        self.add_sphere(sphere1);
        self.add_sphere(sphere2);
        self.add_sphere(sphere3);
        self.add_sphere(sphere4);

        self.set_camera(cam);
        self.add_lightsource(ambient_light);
        self.add_lightsource(point_light);
    }
    /// ## Returns
    /// a mutable reference to the camera
    pub fn get_camera_mut(&mut self) -> &mut Camera {
        self.scene_graph.get_camera_mut()
    }
    /// ## Returns
    ///  a reference to the camera
    pub fn get_camera(&self) -> &Camera {
        self.scene_graph.get_camera()
    }
    /// ## Returns
    /// A new scene with default values.
    ///
    /// If the `CI` or `RENDERBABY_HEADLESS` environment variable is set,
    /// the render engine will not be initialized, allowing usage in headless environments.
    pub fn new() -> Self {
        let headless = std::env::var("CI").is_ok() || std::env::var("RENDERBABY_HEADLESS").is_ok();
        Self::new_with_options(!headless)
    }
    /// Creates a new scene
    /// ## Parameter
    /// 'load_engine': if a render engine is to be loaded. See also function new
    pub fn new_with_options(load_engine: bool) -> Self {
        let cam = Camera::default();
        let Resolution { width, height } = cam.get_resolution();
        let position = cam.get_position();
        let rotation = crate::data_plane::scene::scene_engine_adapter::RenderCamera::default().dir; //Engine uses currently a direction vector
        let pane_width =
            crate::data_plane::scene::scene_engine_adapter::RenderCamera::default().pane_width;
        let render_camera = crate::data_plane::scene::scene_engine_adapter::RenderCamera::new(
            cam.get_fov(),
            pane_width,
            [position.x, position.y, position.z],
            rotation,
        );
        let render_param = RenderParameter::default();
        Self {
            scene_graph: SceneGraph::new(),
            name: "scene".to_owned(),
            render_params: render_param,
            render_engine: if load_engine {
                Option::from(Engine::new(
                    RenderConfigBuilder::new()
                        .uniforms_create(Uniforms::new(
                            *width,
                            *height,
                            render_camera,
                            cam.get_ray_samples(),
                            0,
                            0,
                            0,
                            render_param.ground_height, // Leave or change to scene defaults
                            render_param.ground_enabled,
                            render_param.checkerboard_enabled,
                            render_param.sky_color.into(),
                            render_param.max_depth,
                            render_param.checkerboard_colors.0.into(),
                            render_param.checkerboard_colors.1.into(),
                        ))
                        .spheres_create(vec![])
                        .uvs_create(vec![])
                        .meshes_create(vec![])
                        .lights_create(vec![])
                        .textures_create(vec![])
                        .build(),
                    RenderEngine::Pathtracer,
                ))
            } else {
                None
            },
            first_render: true,
            last_frame: None,
            texture_cache: TextureCache::new(),
            output_path: None,
        }
    }
    /// adds an sphere to the scene
    /// ## Arguments
    /// 'sphere': GeometricObject that is to be added to the scene
    pub fn add_sphere(&mut self, sphere: Sphere) {
        info!("{self}: adding {:?}", sphere);
        self.scene_graph.add_sphere(sphere);
    }
    /// adds an object to the scene
    /// ## Arguments
    /// 'mesh': GeometricObject that is to be added to the scene
    pub fn add_mesh(&mut self, mesh: Mesh) {
        info!("{self}: adding {:?}", mesh.get_name());
        self.scene_graph.add_mesh(mesh);
    }
    /// adds an LightSource to the scene
    /// ## Arguments
    /// 'light': LightSource that is to be added
    pub fn add_lightsource(&mut self, light: LightSource) {
        info!("{self}: adding LightSource {light}");
        self.scene_graph.add_lightsource(light);
    }

    /// deletes all spheres in the scene
    pub fn clear_spheres(&mut self) {
        self.scene_graph.clear_spheres();
    }
    /// deletes all meshes in the scene
    pub fn clear_polygons(&mut self) {
        self.scene_graph.clear_meshes();
    }
    /// sets the scene camera to the passed camera
    /// ## Arguments
    /// 'camera': Camera that is to be the new scene camera
    pub fn set_camera(&mut self, camera: Camera) {
        info!("{self}: set camera to {camera}");
        self.scene_graph.set_camera(camera);
    }
    /// ##  Returns
    /// a reference to a vector of all spheres
    pub fn get_spheres(&self) -> &Vec<Sphere> {
        self.scene_graph.get_spheres()
    }
    /// ##  Returns
    /// a reference to a vector of all spheres
    pub fn get_spheres_mut(&mut self) -> &mut Vec<Sphere> {
        self.scene_graph.get_spheres_mut()
    }
    /// ##  Returns
    /// a reference to a vector of all Meshes
    pub fn get_meshes(&self) -> &Vec<Mesh> {
        self.scene_graph.get_meshes()
    }
    /// ##  Returns
    /// a reference to a vector of all Meshes
    pub fn get_meshes_mut(&mut self) -> &mut Vec<Mesh> {
        self.scene_graph.get_meshes_mut()
    }
    /// ## Returns
    /// Reference to a vector that holds all LightSources of the scene
    pub fn get_light_sources(&self) -> &Vec<LightSource> {
        self.scene_graph.get_light_sources()
    }
    /// ## Returns
    /// Reference to a vector that holds all LightSources of the scene
    pub fn get_light_sources_mut(&mut self) -> &mut Vec<LightSource> {
        self.scene_graph.get_light_sources_mut()
    }
    /// ## Returns
    /// Reference to the scene Engine
    pub fn get_render_engine(&self) -> &Engine {
        self.render_engine.as_ref().expect("No render engine found")
    }
    /// ## Returns
    /// Mutable reference to the scene Engine
    pub fn get_render_engine_mut(&mut self) -> &mut Engine {
        self.render_engine.as_mut().expect("No render engine found")
    }
    /// set the scene engine to the passed scene
    /// ## Arguments
    /// 'engine': engine that will be the new engine
    pub fn set_render_engine(&mut self, engine: Engine) {
        info!(
            "{self}: setting render engine to new {:?}",
            engine.current_engine()
        );
        self.render_engine = Some(engine);
    }

    /// Sets the color_hash_enabled to the given bool. The color hash crates a color for trinagles and can be used if they have no material
    /// ## Parameter
    /// 'enabled': new bool value
    pub fn set_color_hash_enabled(&mut self, enabled: bool) {
        self.render_params.color_hash_enabled = enabled;
        info!("{self}: set color hash enabled to {enabled}");
    }
    /// ## Returns
    /// scene get_color_hash_enabled value. The color hash crates a color for trinagles and can be used if they have no material
    pub fn get_color_hash_enabled(&self) -> bool {
        self.render_params.color_hash_enabled
    }
    /// ## Returns
    /// Reference to the scene name
    pub fn get_name(&self) -> &String {
        &self.name
    }
    /// ## Arguments
    /// 'name' : new scene name
    pub fn set_name(&mut self, name: String) {
        let old_name = self.name.clone();
        self.name = name.clone();
        info!("{self}: Renamed to {name} from {old_name}");
    }
    /// ## Returns
    /// Background color rgb as array of f32
    pub fn get_background_color(&self) -> Color {
        self.render_params.sky_color
    }
    /// ## Parameters
    /// New background color as array of f32
    pub fn set_background_color(&mut self, color: Color) {
        self.render_params.sky_color = color;
        info!(
            "Scene {self}: set background color to [{}, {}, {}]",
            color.r, color.g, color.b
        );
    }
    /// ## Returns
    /// Scene ground height as f32
    pub fn get_ground_height(&self) -> f32 {
        self.render_params.ground_height
    }
    /// ## Parameters
    /// New background color as array of f32
    pub fn set_ground_height(&mut self, height: f32) {
        self.render_params.ground_height = height;
        info!("Scene {self}: set ground height to {}", height);
    }
    /// ## Returns
    /// If ground is anabled
    pub fn get_ground_enabled(&self) -> bool {
        self.render_params.ground_enabled
    }
    /// ## Parameters
    /// 'enabled': bool representing if ground should be enabled or not
    pub fn set_ground_enabled(&mut self, enabled: bool) {
        self.render_params.ground_enabled = enabled;
        info!("Scene {self}: set ground enabled  to {}", enabled);
    }
    /// ## Returns
    /// If checkerboard is enabled
    pub fn get_checkerboard_enabled(&self) -> bool {
        self.render_params.checkerboard_enabled
    }
    /// ## Parameters
    /// 'enabled': bool representing if checkerboard should be enabled or not
    pub fn set_checkerboard_enabled(&mut self, enabled: bool) {
        self.render_params.checkerboard_enabled = enabled;
        info!("Scene {self}: set checkerboard enabled  to {}", enabled);
    }
    /// ## Returns
    /// checkerboard colors as pair of [f32;3]
    pub fn get_checkerboard_colors(&self) -> (Color, Color) {
        self.render_params.checkerboard_colors
    }
    /// ## Parameters
    /// 'colors': pair of [f32;3] representing rgb colors
    pub fn set_checkerboared_colors(&mut self, colors: (Color, Color)) {
        self.render_params.checkerboard_colors = colors;
        info!("Scene {self}: set ground enabled  to {:?}", colors);
    }
    /// ## Returns
    /// Max depth of render recursion
    pub fn get_max_depth(&self) -> u32 {
        self.render_params.max_depth
    }
    /// ## Parameters
    /// 'depth': new maximum depth of render recursions
    pub fn set_max_depth(&mut self, depth: u32) {
        // todo maybe specify valid values? is 1 ok?
        if depth > 0 {
            self.render_params.max_depth = depth;
            info!("Scene {self}: set maximum depth  to {}", depth);
        } else {
            warn!("{self}: ignoring invalid render depth {depth}")
        }
    }
    /// ## Returns
    /// RenderParameter of the scene
    pub fn get_render_parameter(&self) -> RenderParameter {
        self.render_params
    }
    /// ## Parameters
    /// 'param': new RenderParameter
    pub fn set_render_parameter(&mut self, param: RenderParameter) {
        self.render_params = param;
        info!("Scene {self}: set render parameter  to {:?}", param);
    }
    /// Sets the value of field last render to the given Frame
    /// ## Parameter
    /// 'frame': Frame that will be the new value of the field
    pub fn set_last_render(&mut self, frame: Frame) {
        self.last_frame = Some(frame);
        info!("{self}: Last render saved to buffer");
    }
    /// Sets first_render to the passed value
    /// ## Parameter
    /// 'first_render': boolean value
    pub fn set_first_render(&mut self, first_render: bool) {
        self.first_render = first_render
    }
    /// ## Returns
    /// first_render: if a upcoming rendering will be the first
    pub fn get_first_render(&self) -> bool {
        self.first_render
    }

    /// ## Returns
    /// The output path of the scene. This is used for convenience when opening file browsers
    pub fn get_output_path(&self) -> Option<PathBuf> {
        self.output_path.clone()
    }
    /// Exports the last render result to the given path
    /// ## Parameter
    /// 'path': std::path::BathBuf of where the image will be saved
    pub fn export_render_img(&self, path: PathBuf) -> anyhow::Result<()> {
        let render = self.last_frame.clone().ok_or_else(|| {
            image::ImageError::Parameter(image::error::ParameterError::from_kind(
                image::error::ParameterErrorKind::Generic("No render available".into()),
            ))
        })?;

        info!("{self}: Saved image to {:?}", path.clone());
        export_img_png(path, render)?;
        Ok(())
    }
}

impl std::fmt::Display for Scene {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Scene {}", self.get_name())
    }
}
