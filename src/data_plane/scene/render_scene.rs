use std::path::PathBuf;
use anyhow::Error;
use engine_config::{RenderConfigBuilder, Uniforms, TextureData};
use std::collections::HashMap;
use glam::Vec3;
use log::{info, error, debug};
use frame_buffer::frame_iterator::Frame;
use scene_objects::{
    camera::{Camera, Resolution},
    geometric_object::GeometricObject,
    light_source::{LightSource, LightType},
    material::Material,
    mesh::Mesh,
    sphere::Sphere,
};
use crate::{
    compute_plane::{engine::Engine, render_engine::RenderEngine},
    data_plane::{
        scene::{render_parameter::RenderParameter, scene_graph::SceneGraph},
        scene_io::{
            img_export::export_img_png, obj_parser::OBJParser, scene_importer::parse_scene,
        },
    },
};
use crate::data_plane::scene_io::{mtl_parser, scene_exporter};

/// The scene holds all relevant objects, lightsources, camera
pub struct Scene {
    scene_graph: SceneGraph,
    //background_color: [f32; 3],
    name: String,
    render_engine: Option<Engine>,
    first_render: bool,
    last_frame: Option<Frame>,
    pub textures: HashMap<String, TextureData>,
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
    fn _load_scene_from_path(path: PathBuf) -> anyhow::Result<Scene> {
        //! loads and returns a new scene from a json / rscn file at path
        info!("Scene: Loading new scene from {}", path.display());

        let loaded_data = parse_scene(path.clone(), None)?;

        let mut scene = loaded_data.scene;
        let paths = loaded_data.paths;
        let rotation = loaded_data.rotations;
        let translation = loaded_data.translations;
        let scale = loaded_data.scales;

        debug!("Scene: Loading {} objects...", paths.len());
        for (i, p_str) in paths.iter().enumerate() {
            let p = PathBuf::from(p_str);
            debug!("Scene: Loading object {} from {:?}", i, p);
            scene.load_object_from_file_relative(
                p.clone(),
                p,
                translation[i],
                rotation[i],
                scale[i],
            )?;
        }

        info!("Scene: Successfully loaded scene.");
        Ok(scene)
    }

    pub fn export_scene(&self, path: PathBuf) -> Result<(), Error> {
        info!("{self}: Exporting scene");
        let result = scene_exporter::serialize_scene(path.clone(), self);
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

    pub fn parse_obj_to_mesh(
        &mut self,
        path: PathBuf,
        relative_path: Option<PathBuf>,
    ) -> Result<Mesh, Error> {
        //! Adds new object from a obj file at path
        //! ## Parameter
        //! 'path': Path to the obj file
        //! ## Returns
        //! Result of either a reference to the new object or an error
        let parent_dir = path.parent().unwrap_or(std::path::Path::new("."));
        info!("{self}: Loading object from {}", path.display());
        let result = OBJParser::parse(path.clone());

        match result {
            Ok(objs) => {
                debug!(
                    "{self}: Parsed OBJ with {} faces, {} vertices",
                    objs.faces.len(),
                    objs.vertices.len()
                );
                let mut material_name_list: Vec<String> = Vec::new();
                let mut material_list: Vec<Material> = Vec::new();

                if let Some(obj) = objs.material_path {
                    for i in obj {
                        let mtl_path = parent_dir.join(&i);
                        let parsed = mtl_parser::MTLParser::parse(mtl_path.to_str().unwrap_or(&i));
                        match parsed {
                            Ok(parsed) => parsed.iter().for_each(|mat| {
                                // Load texture if present
                                if let Some(tex_name) = &mat.map_kd {
                                    let tex_path = parent_dir.join(tex_name);
                                    let tex_key = tex_path.to_string_lossy().to_string();

                                    if let std::collections::hash_map::Entry::Vacant(e) =
                                        self.textures.entry(tex_key)
                                    {
                                        info!("Loading texture from {:?}", tex_path);
                                        match image::open(&tex_path) {
                                            Ok(img) => {
                                                let img = img.to_rgba8();
                                                let (width, height) = img.dimensions();
                                                let data: Vec<u32> = img
                                                    .pixels()
                                                    .map(|p| u32::from_le_bytes(p.0))
                                                    .collect();

                                                e.insert(TextureData::new(width, height, data));
                                            }
                                            Err(e) => error!(
                                                "Failed to load texture {:?}: {}",
                                                tex_path, e
                                            ),
                                        }
                                    }
                                }

                                material_list.push(Material::new(
                                    mat.name.clone(),
                                    mat.ka.iter().map(|a| *a as f64).collect(),
                                    mat.kd.iter().map(|a| *a as f64).collect(),
                                    mat.ks.iter().map(|a| *a as f64).collect(),
                                    mat.ke.iter().map(|a| *a as f64).collect(),
                                    mat.ns.into(),
                                    mat.d.into(),
                                    mat.map_kd.clone().map(|name| {
                                        parent_dir.join(name).to_string_lossy().to_string()
                                    }),
                                ))
                            }),
                            Err(error) => {
                                info!("{self}: Parsing mtl from {i} resulted in error: {error}");
                            }
                        }
                    }

                    material_list
                        .iter()
                        .for_each(|mat| material_name_list.push(mat.name.clone()));
                }

                let mut new_vertices = Vec::with_capacity(objs.faces.len() * 9);
                let mut new_tris = Vec::with_capacity(objs.faces.len() * 3);
                let mut new_uvs = Vec::with_capacity(objs.faces.len() * 6);
                let mut material_index = Vec::with_capacity(objs.faces.len());

                let mut vertex_count = 0;

                for face in objs.faces {
                    let leng = face.v.len();
                    for i in 1..(leng - 1) {
                        // Get indices for the triangle (0, i, i+1)
                        let v_indices = [0, i, i + 1];

                        for &idx in &v_indices {
                            // Position
                            let v_idx = face.v[idx] as usize - 1;
                            if v_idx * 3 + 2 < objs.vertices.len() {
                                new_vertices.push(objs.vertices[v_idx * 3]);
                                new_vertices.push(objs.vertices[v_idx * 3 + 1]);
                                new_vertices.push(objs.vertices[v_idx * 3 + 2]);
                            } else {
                                // Fallback if index is out of bounds (shouldn't happen with valid OBJ)
                                new_vertices.push(0.0);
                                new_vertices.push(0.0);
                                new_vertices.push(0.0);
                            }

                            // UV
                            if !face.vt.is_empty() && idx < face.vt.len() {
                                let vt_val = face.vt[idx] as usize;
                                if vt_val > 0 {
                                    let vt_idx = vt_val - 1;
                                    if let Some(tex_coords) = &objs.texture_coordinate {
                                        if vt_idx * 2 + 1 < tex_coords.len() {
                                            new_uvs.push(tex_coords[vt_idx * 2]);
                                            new_uvs.push(tex_coords[vt_idx * 2 + 1]);
                                        } else {
                                            new_uvs.push(0.0);
                                            new_uvs.push(0.0);
                                        }
                                    } else {
                                        new_uvs.push(0.0);
                                        new_uvs.push(0.0);
                                    }
                                } else {
                                    new_uvs.push(0.0);
                                    new_uvs.push(0.0);
                                }
                            } else {
                                new_uvs.push(0.0);
                                new_uvs.push(0.0);
                            }

                            // Index
                            new_tris.push(vertex_count);
                            vertex_count += 1;
                        }

                        if let Some(m) = material_list
                            .iter()
                            .position(|x| x.name == face.material_name.clone())
                        {
                            material_index.push(m);
                        }
                    }
                }
                let mut used_path: PathBuf = if let Some(relative_path) = relative_path {
                    relative_path
                } else {
                    path.clone()
                };
                let mesh = Mesh::new(
                    new_vertices,
                    new_tris,
                    if !new_uvs.is_empty() {
                        Some(new_uvs)
                    } else {
                        None
                    },
                    Some(material_list),
                    Some(material_index),
                    Some(objs.name),
                    Some(used_path),
                )?;
                info!(
                    "Scene {self}: Successfully loaded object from {}",
                    path.display()
                );
                Ok(mesh)
            }

            Err(error) => {
                error!(
                    "{self}: Parsing obj from {} resulted in error: {error}",
                    path.display()
                );
                Err(error.into())
            }
        }
    }

    pub fn load_object_from_file(&mut self, path: PathBuf) -> Result<(), Error> {
        let mesh = self.parse_obj_to_mesh(path, None)?;
        self.add_mesh(mesh);
        Ok(())
    }

    fn load_object_from_file_relative(
        &mut self,
        path: PathBuf,
        relative_path: PathBuf,
        translation: Vec3,
        rotation: Vec3,
        scale: Vec3,
    ) -> Result<(), Error> {
        let mut mesh = self.parse_obj_to_mesh(path, Some(relative_path))?;
        mesh.scale(scale.x);
        mesh.rotate(rotation);
        mesh.translate(translation);
        self.add_mesh(mesh);
        Ok(())
    }

    pub fn load_object_from_file_transformed(
        &mut self,
        path: PathBuf,
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

    pub fn load_scene_from_path(path: PathBuf, detached: bool) -> anyhow::Result<Scene> {
        let mut scene = Self::_load_scene_from_path(path.clone());
        match scene {
            Ok(mut scene) => {
                let is_rscn = path
                    .extension()
                    .map(|s| s.to_string_lossy() == "rscn")
                    .unwrap_or(false);

                if !is_rscn && !detached {
                    scene.output_path = Some(path);
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

    pub fn load_scene_from_string(json_string: String) -> anyhow::Result<Scene> {
        let scene = parse_scene(PathBuf::new(), Some(json_string));
        match scene {
            Ok(loaded_data) => {
                let mut scene = loaded_data.scene;
                let paths = loaded_data.paths;
                let rotation = loaded_data.rotations;
                let translation = loaded_data.translations;
                let scale = loaded_data.scales;
                Ok(scene)
            }
            Err(error) => Err(error),
        }
    }

    pub fn save(&mut self) -> anyhow::Result<()> {
        if let Some(output_path) = self.output_path.clone()
        // && output_path.exists() TODO: why would the path already need to exist?
        {
            self.export_scene(output_path)?;
            Ok(())
        } else {
            Err(anyhow::Error::msg(
                "No valid output path set for this scene",
            ))
        }
    }

    pub fn set_output_path(&mut self, path: Option<PathBuf>) -> anyhow::Result<()> {
        self.output_path = path;
        Ok(())
    }

    pub fn proto_init(&mut self) {
        //! For the early version: This function adds a sphere, a camera, and a lightsource
        //! This is a temporary function for test purposes
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
            LightType::Ambient,
        );
        let point_light = LightSource::new(
            Vec3::new(2.0, 4.0, 1.0),
            20.0,
            [1.0, 0.9, 0.8],
            "point_light".to_owned(),
            Vec3::default(),
            LightType::Point,
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

    pub fn get_camera_mut(&mut self) -> &mut Camera {
        //! ## Returns
        //! a mutable reference to the camera
        self.scene_graph.get_camera_mut()
    }

    pub fn get_camera(&self) -> &Camera {
        //! ## Returns
        //!  a reference to the camera
        self.scene_graph.get_camera()
    }

    pub fn new() -> Self {
        //! ## Returns
        //! A new scene with default values.
        //!
        //! If the `CI` or `RENDERBABY_HEADLESS` environment variable is set,
        //! the render engine will not be initialized, allowing usage in headless environments.
        let headless = std::env::var("CI").is_ok() || std::env::var("RENDERBABY_HEADLESS").is_ok();
        Self::new_with_options(!headless)
    }

    pub fn new_with_options(load_engine: bool) -> Self {
        //! Creates a new scene
        //! ## Parameter
        //! 'load_engine': if a render engine is to be loaded. See also function new
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
            //background_color: [1.0, 1.0, 1.0],
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
                            render_param.ground_height, //Leave or change to scene defaults
                            render_param.ground_enabled,
                            render_param.checkerboard_enabled,
                            render_param.sky_color,
                            render_param.max_depth,
                            render_param.checkerboard_colors.0,
                            render_param.checkerboard_colors.1,
                        ))
                        .spheres_create(vec![])
                        .uvs_create(vec![])
                        .meshes_create(vec![])
                        .lights_create(vec![])
                        .textures_create(vec![])
                        .build(),
                    RenderEngine::Raytracer,
                ))
            } else {
                None
            },
            first_render: true,
            last_frame: None,
            textures: HashMap::new(),
            output_path: None,
        }
    }
    pub fn add_sphere(&mut self, sphere: Sphere) {
        //! adds an object to the scene
        //! ## Arguments
        //! 'sphere': GeometricObject that is to be added to the scene
        info!("{self}: adding {:?}", sphere);
        self.scene_graph.add_sphere(sphere);
    }
    pub fn add_mesh(&mut self, mesh: Mesh) {
        //! adds an object to the scene
        //! ## Arguments
        //! 'mesh': GeometricObject that is to be added to the scene
        info!("{self}: adding {:?}", mesh.get_name());
        self.scene_graph.add_mesh(mesh);
    }

    pub fn add_lightsource(&mut self, light: LightSource) {
        //! adds an LightSource to the scene
        //! ## Arguments
        //! 'light': LightSource that is to be added
        info!("{self}: adding LightSource {light}");
        self.scene_graph.add_lightsource(light);
    }

    pub fn clear_spheres(&mut self) {
        self.scene_graph.clear_spheres();
    }

    pub fn clear_polygons(&mut self) {
        self.scene_graph.clear_meshes();
    }
    pub fn set_camera(&mut self, camera: Camera) {
        //! sets the scene camera to the passed camera
        //! ## Arguments
        //! 'camera': Camera that is to be the new scene camera
        info!("{self}: set camera to {camera}");
        self.scene_graph.set_camera(camera);
    }

    pub fn get_spheres(&self) -> &Vec<Sphere> {
        //! ##  Returns
        //! a reference to a vector of all spheres

        self.scene_graph.get_spheres()
    }

    pub fn get_spheres_mut(&mut self) -> &mut Vec<Sphere> {
        //! ##  Returns
        //! a reference to a vector of all spheres

        self.scene_graph.get_spheres_mut()
    }

    pub fn get_meshes(&self) -> &Vec<Mesh> {
        //! ##  Returns
        //! a reference to a vector of all Meshes

        self.scene_graph.get_meshes()
    }

    pub fn get_meshes_mut(&mut self) -> &mut Vec<Mesh> {
        //! ##  Returns
        //! a reference to a vector of all Meshes

        self.scene_graph.get_meshes_mut()
    }

    pub fn get_light_sources(&self) -> &Vec<LightSource> {
        //! ## Returns
        //! Reference to a vector that holds all LightSources of the scene
        self.scene_graph.get_light_sources()
    }

    pub fn get_light_sources_mut(&mut self) -> &mut Vec<LightSource> {
        //! ## Returns
        //! Reference to a vector that holds all LightSources of the scene
        self.scene_graph.get_light_sources_mut()
    }

    pub fn get_render_engine(&self) -> &Engine {
        //! ## Returns
        //! Reference to the scene Engine
        self.render_engine.as_ref().expect("No render engine found")
    }

    pub fn get_render_engine_mut(&mut self) -> &mut Engine {
        //! ## Returns
        //! Mutable reference to the scene Engine
        self.render_engine.as_mut().expect("No render engine found")
    }

    pub fn set_render_engine(&mut self, engine: Engine) {
        //! set the scene engine to the passed scene
        //! ## Arguments
        //! 'engine': engine that will be the new engine
        //!
        info!(
            "{self}: setting render engine to new {:?}",
            engine.current_engine()
        );
        self.render_engine = Some(engine);
    }

    pub fn set_color_hash_enabled(&mut self, enabled: bool) {
        self.render_params.color_hash_enabled = enabled;
        info!("{self}: set color hash enabled to {enabled}");
    }

    pub fn get_color_hash_enabled(&self) -> bool {
        self.render_params.color_hash_enabled
    }

    pub fn get_name(&self) -> &String {
        //!## Returns
        //! Reference to the scene name
        &self.name
    }

    pub fn set_name(&mut self, name: String) {
        //! ## Arguments
        //! 'name' : new scene name
        let old_name = self.name.clone();
        self.name = name.clone();
        info!("{self}: Renamed to {name} from {old_name}");
    }

    pub fn get_background_color(&self) -> [f32; 3] {
        //! ## Returns
        //! Background color rgb as array of f32
        self.render_params.sky_color
    }

    pub fn set_background_color(&mut self, color: [f32; 3]) {
        //! ## Parameters
        //! New background color as array of f32
        self.render_params.sky_color = color;
        info!(
            "Scene {self}: set background color to [{}, {}, {}]",
            color[0], color[1], color[2]
        );
    }

    pub fn set_last_render(&mut self, frame: Frame) {
        self.last_frame = Some(frame);
        info!("{self}: Last render saved to buffer");
    }

    pub fn set_first_render(&mut self, first_render: bool) {
        //! Sets first_render to the passed value
        //! ## Parameter
        //! 'first_render': boolean value
        self.first_render = first_render
    }

    pub fn get_first_render(&self) -> bool {
        //! ## Returns
        //! first_render: if the last render was the first render of this scene?
        self.first_render
    }

    pub fn get_output_path(&self) -> Option<PathBuf> {
        self.output_path.clone()
    }

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
