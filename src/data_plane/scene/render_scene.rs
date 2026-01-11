use std::{backtrace::Backtrace, path::PathBuf};
use anyhow::{anyhow, Error};
use engine_config::{RenderConfig, RenderConfigBuilder, RenderOutput, TextureData, Uniforms};
use std::collections::HashMap;
use glam::Vec3;
use log::{debug, error, info};
use scene_objects::{
    camera::{Camera, Resolution},
    geometric_object::{GeometricObject, SceneObject},
    light_source::{LightSource, LightType},
    material::Material,
    mesh::Mesh,
    sphere::Sphere,
};
use crate::{
    compute_plane::{engine::Engine, render_engine::RenderEngine},
    data_plane::{
        scene::{
            scene_change::{CameraChange, SceneChange},
            scene_engine_adapter::{
                camera_to_render_uniforms, light_to_render_point_light, mesh_to_render_data,
                sphere_to_render_sphere,
            },
            scene_graph::SceneGraph,
        },
        scene_io::{
            img_export::export_img_png, obj_parser::OBJParser, scene_importer::parse_scene,
        },
    },
};
use crate::data_plane::scene_io::{mtl_parser, scene_exporter};

type RenderSphere = engine_config::Sphere;
type RenderUniforms = engine_config::Uniforms;
type RenderMesh = engine_config::Mesh;
pub type RenderCamera = engine_config::Camera;
type RenderLight = engine_config::PointLight;
type RenderGeometry = (Vec<f32>, Vec<u32>, Vec<f32>, engine_config::Material);

/// The scene holds all relevant objects, lightsources, camera
pub struct Scene {
    scene_graph: SceneGraph,
    background_color: [f32; 3],
    name: String,
    render_engine: Option<Engine>,
    render_config_builder: RenderConfigBuilder,
    first_render: bool,
    last_render: Option<RenderOutput>,
    color_hash_enabled: bool,
    pub textures: HashMap<String, TextureData>,
}
impl Default for Scene {
    fn default() -> Self {
        Self::new()
    }
}
#[allow(dead_code)]
impl Scene {
    /// loads and return a new scene from a json / rscn file
    pub(crate) fn load_scene_from_file(path: PathBuf) -> anyhow::Result<Scene> {
        let mut directory_path = path.clone();
        directory_path.pop();
        info!("Scene: Loading new scene from {}", path.display());
        let scene_and_path = parse_scene(path.clone());
        match scene_and_path {
            Ok(scene_and_path) => {
                let mut scene = scene_and_path.0;
                let paths = scene_and_path.1;
                let mut pathbuf = Vec::with_capacity(1);
                paths
                    .iter()
                    .for_each(|path| pathbuf.push(directory_path.join(path)));
                for (i, v) in pathbuf.iter().enumerate() {
                    scene.load_object_from_file_relative(
                        v.clone(),
                        PathBuf::from(paths[i].clone()),
                    )?;
                }
                Ok(scene)
            }
            Err(error) => {
                error!("Scene: Importing Scene resulted in error: {error}");
                Err(error)
            }
        }
    }
    pub fn export_scene(&self, path: PathBuf) -> Result<(), Error> {
        info!("Scene {self}: Exporting scene");
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
                info!(
                    "Scene {self}: Successfully exported scene to {}",
                    path.display()
                );
                Ok(())
            }
        }
    }

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
        info!("Scene {self}: Loading object from {}", path.display());
        let result = OBJParser::parse(path.clone());

        match result {
            Ok(objs) => {
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
                let used_path: PathBuf = if let Some(relative_path) = relative_path {
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
    ) -> Result<(), Error> {
        let mesh = self.parse_obj_to_mesh(path, Some(relative_path))?;
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

    pub(in crate::data_plane) fn get_camera_mut(&mut self) -> &mut Camera {
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
        //! A new scene with default values
        let cam = Camera::default();
        let Resolution { width, height } = cam.get_resolution();
        let position = cam.get_position();
        let rotation = RenderCamera::default().dir; //Engine uses currently a direction vector
        let pane_width = RenderCamera::default().pane_width;
        let render_camera = RenderCamera::new(
            cam.get_pane_distance(),
            pane_width,
            [position.x, position.y, position.z],
            rotation,
        );
        let mut res = Self {
            scene_graph: SceneGraph::new(),
            name: "scene".to_owned(),
            background_color: [1.0, 1.0, 1.0],
            render_engine: None,
            render_config_builder: RenderConfigBuilder::new(),
            first_render: true,
            last_render: None,
            color_hash_enabled: true,
            textures: HashMap::new(),
        };
        res.render_config_builder = RenderConfigBuilder::new()
            .uniforms_create(Uniforms::new(
                *width,
                *height,
                render_camera,
                cam.get_ray_samples(),
                0,
                0,
            ))
            .spheres_create(vec![])
            .vertices_create(vec![])
            .uvs_create(vec![])
            .triangles_create(vec![])
            .meshes_create(vec![])
            .lights_create(vec![])
            .textures_create(vec![]);
        res.set_render_engine(Engine::new(
            res.build_render_config(),
            RenderEngine::Raytracer,
        ));
        // render one time so that everything in the config is created
        match res.render() {
            Ok(_) => debug!("Successfull initial render on new scene"),
            Err(err) => error!("Failure during initial render of new scene : {err}"),
        }

        res
    }
    pub fn proto_init(&mut self) {
        //! For the early version: This function adds a sphere, a camera, and a lightsource
        //! This is a temporary function for test purposes
        info!("{self}: Initialising with 'proto' settings");
        /* let green = [0.0, 1.0, 0.0];
        let red = [1.0, 0.0, 0.0];
        let blue = [0.0, 0.0, 1.0];
        let cyan = [0.0, 1.0, 1.0]; */
        let magenta = [1.0, 0.0, 1.0];

        let sphere0 = Sphere::new(Vec3::new(0.0, 0.6, 2.0), 0.5, Material::default(), magenta);
        /* let sphere1 = Sphere::new(Vec3::new(-0.6, 0.0, 2.0), 0.5, Material::default(), green);
        let sphere2 = Sphere::new(Vec3::new(0.0, 0.0, 2.0), 0.5, Material::default(), red);
        let sphere3 = Sphere::new(Vec3::new(0.6, 0.0, 2.0), 0.5, Material::default(), blue);
        let sphere4 = Sphere::new(Vec3::new(0.0, -0.6, 2.0), 0.5, Material::default(), cyan); */

        let cam = Camera::default();
        let light = LightSource::new(
            Vec3::new(0.0, 0.0, 3.0),
            0.0,
            [1.0, 1.0, 1.0],
            "proto_light".to_owned(),
            Vec3::default(),
            LightType::Ambient,
        );
        self.set_camera(cam);
        self.add_lightsource(light);

        self.add_sphere(sphere0);
        /* self.add_sphere(sphere1);
        self.add_sphere(sphere2);
        self.add_sphere(sphere3);
        self.add_sphere(sphere4); */

        /* self.update_render_config();
        let _ = self.render(); */
    }

    pub fn add_sphere(&mut self, sphere: Sphere) {
        //! adds an object to the scene
        //! ## Arguments
        //! 'sphere': GeometricObject that is to be added to the scene
        info!("{self}: adding {:?}", sphere);
        self.scene_graph.add_sphere(sphere);
        self.update_render_config_spheres();
    }
    pub fn add_mesh(&mut self, mesh: Mesh) {
        //! adds an object to the scene
        //! ## Arguments
        //! 'mesh': GeometricObject that is to be added to the scene
        info!("{self}: adding {:?}", mesh.get_name());
        self.scene_graph.add_mesh(mesh);
        self.update_render_config_triangles();
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
        self.update_render_config_spheres();
        self.update_render_config_uniform();
    }

    pub fn clear_polygons(&mut self) {
        self.scene_graph.clear_meshes();
        self.update_render_config_spheres();
        self.update_render_config_uniform();
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
    pub(in crate::data_plane) fn get_spheres_mut(&mut self) -> &mut Vec<Sphere> {
        //! ##  Returns
        //! a reference to a vector of all spheres

        self.scene_graph.get_spheres_mut()
    }
    pub fn get_meshes(&self) -> &Vec<Mesh> {
        //! ##  Returns
        //! a reference to a vector of all Meshes

        self.scene_graph.get_meshes()
    }
    pub(in crate::data_plane) fn get_meshes_mut(&mut self) -> &mut Vec<Mesh> {
        //! ##  Returns
        //! a reference to a vector of all Meshes

        self.scene_graph.get_meshes_mut()
    }

    pub fn get_light_sources(&self) -> &Vec<LightSource> {
        //! ## Returns
        //! Reference to a vector that holds all LightSources of the scene
        self.scene_graph.get_light_sources()
    }

    pub(in crate::data_plane) fn get_light_sources_mut(&mut self) -> &mut Vec<LightSource> {
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
        self.color_hash_enabled = enabled;
        info!("{self}: set color hash enabled to {enabled}");
        self.update_render_config_uniform();
    }

    pub fn get_color_hash_enabled(&self) -> bool {
        self.color_hash_enabled
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
        self.background_color
    }

    pub fn set_background_color(&mut self, color: [f32; 3]) {
        //! ## Parameters
        //! New background color as array of f32
        self.background_color = color;
        info!(
            "Scene {self}: set background color to [{}, {}, {}]",
            color[0], color[1], color[2]
        );
    }

    // render stuff
    fn get_render_point_lights(&self) -> Vec<RenderLight> {
        //! ## Returns
        //! A vector with all engine_config::PointLight from self
        let mut res = vec![];
        for light in self.get_light_sources() {
            if let Some(render_light) = light_to_render_point_light(light) {
                res.push(render_light);
            }
        }
        res
    }

    fn set_last_render(&mut self, render: RenderOutput) {
        //! Sets the last render field to the given Renderoutput
        //! ## Parameter
        //! 'render': RenderOutput
        self.last_render = Some(render.clone());
        info!("{self}: Last render saved to buffer");
    }

    fn set_first_render(&mut self, first_render: bool) {
        //! Sets first_render to the passed value
        //! ## Parameter
        //! 'first_render': boolean value
        self.first_render = first_render
    }

    fn get_first_render(&self) -> bool {
        //! ## Returns
        //! first_render: if the last render was the first render of this scene?
        self.first_render
    }

    pub fn export_render_img(&self, path: PathBuf) -> anyhow::Result<()> {
        let render = self.last_render.clone().ok_or_else(|| {
            image::ImageError::Parameter(image::error::ParameterError::from_kind(
                image::error::ParameterErrorKind::Generic("No render available".into()),
            ))
        })?;

        info!("{self}: Saved image to {:?}", path.clone());
        export_img_png(path, render)?;
        Ok(())
    }

    fn get_render_spheres(&self) -> Vec<RenderSphere> {
        //! ## Returns
        //! a Vec that contains all Scene spheres as engine_config::Sphere
        self.get_spheres()
            .iter()
            .map(sphere_to_render_sphere)
            .collect()
    }
    fn get_render_uniforms(&self, spheres_count: u32, triangles_count: u32) -> RenderUniforms {
        //! ## Returns
        //! RenderUnfiform for the camera of the scene
        camera_to_render_uniforms(
            self.get_camera(),
            spheres_count,
            triangles_count,
            self.get_color_hash_enabled(),
        )
        .unwrap()
    }

    fn get_render_tris(&self, texture_map: &HashMap<String, i32>) -> Vec<RenderGeometry> {
        //! ## Returns
        //! Vector of touples, with each of the touples representing a TriGeometry defined by the points and the triangles build from the points.
        self.get_meshes()
            .iter()
            .flat_map(|m| mesh_to_render_data(m, texture_map))
            .collect()
    }

    pub fn render(&mut self) -> Result<RenderOutput, Error> {
        //! calls the render engine for the scene self.
        //! ## Returns
        //! Result of either the RenderOutput or a error
        info!("{self}: Render has been called");
        if self.get_first_render() {
            info!(
                "{self}: Render has been called for the first time. Updating entire render config"
            );
            self.update_render_config();
        } else {
            // uniform is not very expensive. Maybe Remove if it doesnt crash the application
            self.update_render_config();
        }

        let rc = self.build_render_config();
        let output = self.get_render_engine_mut().render(rc);
        match output {
            Ok(res) => match res.validate() {
                Ok(_) => {
                    info!("{self}: Successfully got valid render output");
                    self.set_last_render(res.clone());
                    Ok(res)
                }
                Err(error) => {
                    error!("{self}: Received invalid render output");
                    Err(error)
                }
            },
            Err(error) => {
                info!("{:?}", self.render_config_builder);
                println!("Custom backtrace: {}", Backtrace::force_capture());
                error!("{self}: The following error occurred when rendering: {error}");
                Err(error)
            }
        }
    }

    fn build_render_config(&self) -> RenderConfig {
        //! ## Returns
        //! RenderConfig build from self.render_config_builder
        self.render_config_builder.clone().build()
    }
    pub(super) fn update_render_config(&mut self) {
        //! updates the field render_context_builder
        info!("{self}: Updating render config builder");
        let render_spheres = self.get_render_spheres();
        let mut texture_list = Vec::new();
        let mut texture_map = HashMap::new();

        for (path, data) in &self.textures {
            texture_map.insert(path.clone(), texture_list.len() as i32);
            texture_list.push(data.clone());
        }
        let render_tris = self.get_render_tris(&texture_map);
        debug!("Scene mesh data: {:?}", self.get_meshes());
        debug!("Collected mesh data: {:?}", render_tris);

        let spheres_count = render_spheres.len() as u32;
        let triangles_count = render_tris
            .iter()
            .map(|(_, tri, _, _)| tri.len() as u32 / 3)
            .sum();

        let uniforms = self.get_render_uniforms(spheres_count, triangles_count);

        // Collect all vertices and triangles into flat vectors
        let (all_vertices, all_triangles, all_meshes, all_uvs) = if render_tris.is_empty() {
            (vec![], vec![], vec![], vec![])
        } else {
            let mut all_verts = vec![];
            let mut all_tris = vec![];
            let mut all_uvs = vec![];
            let mut mesh_infos = vec![];
            let mut vertex_offset = 0u32;
            let mut triangle_offset = 0u32;

            for (verts, tris, uvs, material) in render_tris.iter() {
                let vertex_count = (verts.len() / 3) as u32;
                let triangle_count = (tris.len() / 3) as u32;

                // Add mesh metadata
                mesh_infos.push(RenderMesh::new(triangle_offset, triangle_count, *material));

                // Add triangles with vertex offset
                for tri_idx in tris {
                    all_tris.push(tri_idx + vertex_offset);
                }

                // Add vertices
                all_verts.extend(verts);

                // Add UVs
                all_uvs.extend(uvs);

                vertex_offset += vertex_count;
                triangle_offset += triangle_count;
            }

            (all_verts, all_tris, mesh_infos, all_uvs)
        };
        debug!("Collected vertices: {:?}", all_vertices);
        debug!("Collected tris: {:?}", all_triangles);
        info!(
            "{self}: Data for render config: {} spheres, {} triangles consisting of {} vertices.",
            render_spheres.len(),
            triangles_count,
            all_vertices.len() / 3
        );
        let point_lights = self.get_render_point_lights();

        self.render_config_builder = if self.get_first_render() {
            info!("Rendering for the first time: Using create for render config");
            self.set_first_render(false);
            // NOTE: *_create is for the first initial render which initializes all the buffers etc.
            RenderConfigBuilder::new()
                .uniforms_create(uniforms)
                .spheres_create(render_spheres)
                .vertices_create(all_vertices)
                .uvs_create(all_uvs)
                .triangles_create(all_triangles)
                .meshes_create(all_meshes)
                .lights_create(point_lights)
                .textures_create(texture_list)
        } else {
            // NOTE: * otherwise the values are updated with the new value and the unchanged fields
            // are kept as is. See: ../../../crates/engine-config/src/render_config.rs - `Change<T>`
            RenderConfigBuilder::new()
                .uniforms(uniforms)
                .spheres(render_spheres)
                .vertices(all_vertices)
                .uvs(all_uvs)
                .triangles(all_triangles)
                .meshes(all_meshes)
                .lights(point_lights)
                .textures(texture_list)
        };
    }
    //todo: check if assignment on self.render_config_builder can be replaced by self.render_config_builder.spheres(...)
    pub(super) fn update_render_config_uniform(&mut self) {
        //! updates the uniforms on field render_config_builder

        let sphere_count = self.get_spheres().len();
        let triangle_count: usize = self
            .get_meshes()
            .iter()
            .map(|mesh| mesh.get_tri_indices().len() / 3)
            .sum();
        let uniforms = self.get_render_uniforms(sphere_count as u32, triangle_count as u32);
        if self.get_first_render() {
            self.update_render_config(); // todo: maybe not entire render config update is needed
            info!("{self}: Change on uniforms before first render: Updating entire render config");
        } else {
            info!("{self}: Updating uniforms on render config builder");
            self.render_config_builder = self.render_config_builder.clone().uniforms(uniforms);
        }
    }
    pub(super) fn update_render_config_spheres(&mut self) {
        //! updates the spheres on field render_config_builder
        info!("{self}: Updating spheres on render config builder");
        let render_spheres = self.get_render_spheres();
        self.render_config_builder = self.render_config_builder.clone().spheres(render_spheres);
    }
    pub(super) fn update_render_config_vertices(&mut self) {
        //! updates the vertices on field render_config_builder
        info!("{self}: Updating vertices on render config builder");
        let mut texture_list = Vec::new();
        let mut texture_map = HashMap::new();

        for (path, data) in &self.textures {
            texture_map.insert(path.clone(), texture_list.len() as i32);
            texture_list.push(data.clone());
        }
        let render_tris = self.get_render_tris(&texture_map);
        debug!("Collected mesh data: {:?}", render_tris);

        // Collect all vertices and triangles into flat vectors
        let all_vertices = if render_tris.is_empty() {
            vec![]
        } else {
            let mut all_verts = vec![];

            for (verts, _, _, _) in render_tris {
                all_verts.extend(verts);
            }
            all_verts
        };
        debug!("Collected vertices: {:?}", all_vertices);
        info!(
            "{self}: Collected render parameter: {} vertices. Updating render config vertices",
            all_vertices.len() / 3
        );
        self.render_config_builder = self.render_config_builder.clone().vertices(all_vertices);
    }
    pub(super) fn update_render_config_triangles(&mut self) {
        //! updates the triangles on field render_config_builder (also updates the vertices?)
        info!("{self}: Updating triangles on render config builder");

        let mut texture_list = Vec::new();
        let mut texture_map = HashMap::new();

        for (path, data) in &self.textures {
            texture_map.insert(path.clone(), texture_list.len() as i32);
            texture_list.push(data.clone());
        }
        let render_tris = self.get_render_tris(&texture_map);
        debug!("Collected mesh data: {:?}", render_tris);

        // Collect all vertices and triangles into flat vectors
        let (all_vertices, all_triangles, all_meshes, all_uvs) = if render_tris.is_empty() {
            (vec![], vec![], vec![], vec![])
        } else {
            let mut all_verts = vec![];
            let mut all_tris = vec![];
            let mut all_uvs = vec![];
            let mut mesh_infos = vec![];
            let mut vertex_offset = 0u32;
            let mut triangle_offset = 0u32;

            for (verts, tris, uvs, material) in render_tris.iter() {
                let vertex_count = (verts.len() / 3) as u32;
                let triangle_count = (tris.len() / 3) as u32;

                // Add mesh metadata
                mesh_infos.push(RenderMesh::new(triangle_offset, triangle_count, *material));

                // Add triangles with vertex offset
                for tri_idx in tris {
                    all_tris.push(tri_idx + vertex_offset);
                }

                // Add vertices
                all_verts.extend(verts);

                // Add UVs
                all_uvs.extend(uvs);

                vertex_offset += vertex_count;
                triangle_offset += triangle_count;
            }

            (all_verts, all_tris, mesh_infos, all_uvs)
        };
        debug!("Collected vertices: {:?}", all_vertices);
        debug!("Collected tris: {:?}", all_triangles);
        info!(
            "{self}: Collected render parameter: {} vertices. Updating render config vertices and triangles",
            all_vertices.len() / 3
        );
        self.render_config_builder = self
            .render_config_builder
            .clone()
            .vertices_create(all_vertices)
            .uvs_create(all_uvs)
            .triangles_create(all_triangles)
            .meshes_create(all_meshes)
            .textures_create(texture_list)
    }

    pub(super) fn update_render_config_lights(&mut self) {
        //! updates the lights on field render_config_builder
        info!("{self}: Updating lights on render config builder");
        let point_lights = self.get_render_point_lights();
        self.render_config_builder = self.render_config_builder.clone().lights(point_lights);
    }

    // camera stuff
    pub(crate) fn set_camera_position(&mut self, position: Vec3) -> Result<(), Error> {
        //! sets the position of the camera
        //! ## Parameter
        //! 'position': glam::Vec3 of the new position
        self.handle_scene_change(SceneChange::CameraChange(CameraChange::Position(position)))
    }
    pub(crate) fn set_camera_look_at(&mut self, look_at: Vec3) -> Result<(), Error> {
        //! sets the direction of the camera
        //! ## Parameter
        //! 'look_at': glam::Vec3 of the new direction
        self.handle_scene_change(SceneChange::CameraChange(CameraChange::LookAt(look_at)))
    }
    pub(crate) fn get_camera_position(&self) -> Vec3 {
        //! ## Returns
        //! Camera position as glam::Vec3
        self.get_camera().get_position()
    }
    pub(crate) fn get_camera_look_at(&self) -> Vec3 {
        //! ## Returns
        //! Camera look at point as glam::Vec3
        self.get_camera().get_look_at()
    }
    pub(crate) fn get_camera_up(&self) -> Vec3 {
        //! ## Returns
        //! up vector of the camera
        self.get_camera().get_up()
    }
    pub(crate) fn set_camera_up(&mut self, up: Vec3) -> Result<(), Error> {
        //! Sets the up vector of the camera to the given value
        //! ## Parameter
        //! 'up': glam::Vec3 for the new vector
        self.handle_scene_change(SceneChange::CameraChange(CameraChange::Up(up)))
    }
    pub(crate) fn get_camera_fov(&self) -> f32 {
        //! ## Returns
        //! Camera field of view, calculated drom width and distance
        //self.fov
        self.get_camera().get_fov()
    }
    pub(crate) fn set_camera_pane_distance(&mut self, distance: f32) -> Result<(), Error> {
        //! Set the camera pane distance
        //! ## Parameter
        //! distance: New value for pane_distance
        self.handle_scene_change(SceneChange::CameraChange(CameraChange::PaneDistance(
            distance,
        )))
    }
    pub(crate) fn get_camera_pane_distance(&self) -> f32 {
        //! ## Returns
        //! Camera pane distance
        self.get_camera().get_pane_distance()
    }
    pub(crate) fn set_camera_pane_width(&mut self, width: f32) -> Result<(), Error> {
        //! Set the camera pane width
        //! ## Parameter
        //! width: New value for pane_distance
        self.handle_scene_change(SceneChange::CameraChange(CameraChange::PaneWidth(width)))
    }
    pub(crate) fn get_camera_pane_width(&self) -> f32 {
        //! ## Returns
        //! Camera pane width
        self.get_camera().get_pane_width()
    }
    pub(crate) fn get_camera_resolution(&self) -> &Resolution {
        //! ## Returns
        //! Camera resolution as Array of u32
        self.get_camera().get_resolution()
    }
    pub(crate) fn set_camera_resolution(&mut self, resolution: Resolution) -> Result<(), Error> {
        //! Sets the camera resolution
        //! ## Parameter
        //! 'resolution': New resolution as array of u32
        self.handle_scene_change(SceneChange::CameraChange(CameraChange::Resolution(
            resolution,
        )))
    }

    pub(crate) fn get_camera_ray_samples(&self) -> u32 {
        //! ## Returns
        //! Scene camera ray samples
        self.get_camera().get_ray_samples()
    }
    pub(crate) fn set_camera_ray_samples(&mut self, samples: u32) -> Result<(), Error> {
        //! Sets the scene camera ray samples to the given value
        //! ## Parameter
        //! 'samples': new ray sample count
        self.handle_scene_change(SceneChange::CameraChange(CameraChange::RaySamples(samples)))
    }

    // sphere stuff
    fn get_sphere_at(&self, index: usize) -> Result<&Sphere, Error> {
        //! private helper fn to get the sphere stored at the given index
        //! ## Parameter
        //! 'index': Index at which the sphere is stored
        //! ## Returns
        //! Result of either the sphere or an error if the index is out of bound
        match self.get_spheres().get(index) {
            Some(sphere) => Ok(sphere),
            None => Err(anyhow!("Index out of bound")),
        }
    }
    fn get_sphere_mut_at(&mut self, index: usize) -> Result<&mut Sphere, Error> {
        //! private helper fn to get the mutable sphere stored at the given index
        //! ## Parameter
        //! 'index': Index at which the sphere is stored
        //! ## Returns
        //! Result of either the sphere or an error if the index is out of bound
        match self.get_spheres_mut().get_mut(index) {
            Some(sphere) => Ok(sphere),
            None => Err(anyhow!("Index out of bound")),
        }
    }
    pub(crate) fn set_sphere_color(&mut self, color: [f32; 3], index: usize) -> Result<(), Error> {
        //! Sets the LightSource color
        //! ## Parameter
        //! 'color': New LightSource color as array of f32, values in \[0, 1]
        self.get_sphere_mut_at(index)?.set_color(color);
        Ok(())
    }
    pub(crate) fn get_sphere_color(&self, index: usize) -> Result<[f32; 3], Error> {
        //! ## Returns
        //! LightSource color as rgb array of f32, values in \[0, 1]
        Ok(self.get_sphere_at(index)?.get_color())
    }
    pub(crate) fn set_sphere_radius(
        &mut self,
        radius: f32,
        index: usize,
    ) -> Result<(), Box<Error>> {
        //! Sets the radius
        //! ## Parameter
        //! 'radius': new radius
        self.get_sphere_mut_at(index)?.set_radius(radius);
        Ok(())
    }

    pub(crate) fn get_sphere_radius(&self, index: usize) -> Result<f32, Error> {
        //! ## Returns
        //! Sphere radius
        Ok(self.get_sphere_at(index)?.get_radius())
    }

    pub(crate) fn get_sphere_center(&self, index: usize) -> Result<Vec3, Error> {
        //! ## Returns
        //! Sphere center as glam::Vec3
        Ok(self.get_sphere_at(index)?.get_center())
    }

    pub(crate) fn set_sphere_center(
        &mut self,
        center: Vec3,
        index: usize,
    ) -> Result<(), Box<Error>> {
        //! sets the Sphere center
        //! ## Parameter
        //! 'center'
        self.get_sphere_mut_at(index)?.set_center(center);
        Ok(())
    }

    pub(crate) fn get_sphere_material(&self, index: usize) -> Result<&Material, Error> {
        //! ## Returns
        //! Reference to Sphere material
        Ok(self.get_sphere_at(index)?.get_material())
    }
    pub(crate) fn set_sphere_material(
        &mut self,
        material: Material,
        index: usize,
    ) -> Result<(), Error> {
        //! Sets the Sphere Material
        //! ## Parameter
        //! 'material': New material
        self.get_sphere_mut_at(index)?.set_material(material);
        Ok(())
    }
    pub(crate) fn get_sphere_path(&self, index: usize) -> Result<Option<PathBuf>, Error> {
        // todo: maybe remove the option?
        //! ## Returns
        //! Path of the reference file. Does a sphere need one?
        Ok(self.get_sphere_at(index)?.get_path())
    }

    pub(crate) fn get_sphere_scale(&self, index: usize) -> Result<Vec3, Error> {
        //! ## Returns
        //! Scale in relation to the reference
        Ok(self.get_sphere_at(index)?.get_scale())
    }

    pub(crate) fn get_sphere_translation(&self, index: usize) -> Result<Vec3, Error> {
        //! ## Returns
        //! Translation in relation to the reference as glam::Vec3
        Ok(self.get_sphere_at(index)?.get_translation())
    }

    pub(crate) fn get_sphere_rotation(&self, index: usize) -> Result<Vec3, Error> {
        //! ## Returns
        //! Rotation in relation
        Ok(self.get_sphere_at(index)?.get_rotation())
    }
    pub(crate) fn scale_sphere(&mut self, factor: f32, index: usize) -> Result<(), Error> {
        //! scales the radius of the sphere
        //! ## Parameter
        //! 'factor': scale factor
        self.get_sphere_mut_at(index)?.scale(factor);
        Ok(())
    }
    pub(crate) fn translate_sphere(&mut self, vec: Vec3, index: usize) -> Result<(), Error> {
        //! Moves the center of the sphere
        //! ## Parameter
        //! 'vec': Translation vector as glam::Vec3
        self.get_sphere_mut_at(index)?.translate(vec);
        Ok(())
    }

    // mesh stuff
    fn get_mesh_at(&self, index: usize) -> Result<&Mesh, Error> {
        //! private helper fn to get the mesh stored at the given index
        //! ## Parameter
        //! 'index': Index at which the mesh is stored
        //! ## Returns
        //! Result of either the mesh or an error if the index is out of bound
        match self.get_meshes().get(index) {
            Some(mesh) => Ok(mesh),
            None => Err(anyhow!("Index out of bound")),
        }
    }
    fn get_mesh_mut_at(&mut self, index: usize) -> Result<&mut Mesh, Error> {
        //! private helper fn to get the mutable mesh stored at the given index
        //! ## Parameter
        //! 'index': Index at which the mesh is stored
        //! ## Returns
        //! Result of either the mesh or an error if the index is out of bound
        match self.get_meshes_mut().get_mut(index) {
            Some(mesh) => Ok(mesh),
            None => Err(anyhow!("Index out of bound")),
        }
    }

    pub(crate) fn get_mesh_path(&self, index: usize) -> Result<Option<PathBuf>, Error> {
        // todo: maybe remove the option?
        //! ## Returns
        //! Path of the reference file. Does a mesh need one?
        Ok(self.get_mesh_at(index)?.get_path())
    }

    pub(crate) fn get_mesh_scale(&self, index: usize) -> Result<Vec3, Error> {
        //! ## Returns
        //! Scale in relation to the reference
        Ok(self.get_mesh_at(index)?.get_scale())
    }

    pub(crate) fn get_mesh_translation(&self, index: usize) -> Result<Vec3, Error> {
        //! ## Returns
        //! Translation in relation to the reference as glam::Vec3
        Ok(self.get_mesh_at(index)?.get_translation())
    }

    pub(crate) fn get_mesh_rotation(&self, index: usize) -> Result<Vec3, Error> {
        //! ## Returns
        //! Rotation in relation
        Ok(self.get_mesh_at(index)?.get_rotation())
    }
    pub(crate) fn scale_mesh(&mut self, factor: f32, index: usize) -> Result<(), Error> {
        //! scales the radius of the mesh
        //! ## Parameter
        //! 'factor': scale factor
        self.get_mesh_mut_at(index)?.scale(factor);
        Ok(())
    }
    pub(crate) fn translate_mesh(&mut self, vec: Vec3, index: usize) -> Result<(), Error> {
        //! Moves the center of the mesh
        //! ## Parameter
        //! 'vec': Translation vector as glam::Vec3
        self.get_mesh_mut_at(index)?.translate(vec);
        Ok(())
    }

    pub(crate) fn rotate_mesh(&mut self, rotation: Vec3, index: usize) -> Result<(), Error> {
        //! Rotates the mesh around its centroid
        //! ## Parameter
        //! 'vec': rotation vector as glam::Vec3, euler angles
        self.get_mesh_mut_at(index)?.translate(rotation);
        Ok(())
    }

    // light stuff: position, luminosity, color, type
    fn get_light_at(&self, index: usize) -> Result<&LightSource, Error> {
        //! ## Returns
        //! Reference to Light at the given index, or Error if index is out of bounds
        match self.get_light_sources().get(index) {
            Some(light_source) => Ok(light_source),
            None => Err(anyhow!("IndexOutOfBound")),
        }
    }
    fn get_light_mut_at(&mut self, index: usize) -> Result<&mut LightSource, Error> {
        //! ## Returns
        //! Mutable Reference to Light at the given index, or Error if index is out of bounds
        match self.get_light_sources_mut().get_mut(index) {
            Some(light_source) => Ok(light_source),
            None => Err(anyhow!("IndexOutOfBound")),
        }
    }
    pub(crate) fn get_light_position(&self, index: usize) -> Result<Vec3, Error> {
        //! ## Returns
        //! Position of light at given index as glam::Vec3
        Ok(self.get_light_at(index)?.get_position())
    }
    pub(crate) fn set_light_position(&mut self, position: Vec3, index: usize) -> Result<(), Error> {
        //!
        self.get_light_mut_at(index)?.set_position(position);
        Ok(())
    }
    pub(crate) fn get_light_luminosity(&self, index: usize) -> Result<f32, Error> {
        //! ## Returns
        //! Luminosity of light at given index
        Ok(self.get_light_at(index)?.get_luminositoy())
    }
    pub(crate) fn set_light_luminosity(
        &mut self,
        luminosity: f32,
        index: usize,
    ) -> Result<(), Error> {
        //!
        self.get_light_mut_at(index)?.set_luminosity(luminosity);
        Ok(())
    }
    pub(crate) fn get_light_color(&self, index: usize) -> Result<[f32; 3], Error> {
        //! ## Returns
        //! Color of light at given index as [f32; 3]
        Ok(self.get_light_at(index)?.get_color())
    }
    pub(crate) fn set_light_color(&mut self, color: [f32; 3], index: usize) -> Result<(), Error> {
        //!
        self.get_light_mut_at(index)?.set_color(color);
        Ok(())
    }
    pub(crate) fn get_light_type(&self, index: usize) -> Result<&LightType, Error> {
        //! ## Returns
        //! Type of light at given index as glam::Vec3
        Ok(self.get_light_at(index)?.get_light_type())
    }
    pub(crate) fn set_light_type(
        &mut self,
        light_type: LightType,
        index: usize,
    ) -> Result<(), Error> {
        //!
        self.get_light_mut_at(index)?.set_light_type(light_type);
        Ok(())
    }
}

impl std::fmt::Display for Scene {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Scene {}", self.get_name())
    }
}
