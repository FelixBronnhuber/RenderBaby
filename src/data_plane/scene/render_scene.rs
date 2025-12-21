use std::path::{PathBuf};
use anyhow::{Error};
use engine_config::{RenderConfigBuilder, Uniforms, RenderOutput};
use glam::Vec3;
use log::{info, error};
use scene_objects::{
    camera::{Camera, Resolution},
    light_source::{LightSource, LightType},
    material::Material,
    mesh::Mesh,
    sphere::Sphere,
};
use serde::Serialize;
use crate::{
    compute_plane::{engine::Engine, render_engine::RenderEngine},
    data_plane::{
        scene::scene_graph::SceneGraph,
        scene_io::{
            obj_parser::OBJParser, scene_importer::parse_scene, img_export::export_img_png,
        },
    },
};
use crate::data_plane::scene_io::mtl_parser;

/// The scene holds all relevant objects, lightsources, camera
#[derive(Serialize)]
pub struct Scene {
    //#[serde(rename(serialize = "items"))]
    #[serde(flatten)]
    scene_graph: SceneGraph,
    background_color: [f32; 3],
    name: String,
    #[serde(skip_serializing)]
    render_engine: Option<Engine>,
    #[serde(skip_serializing)]
    pub(crate) first_render: bool,
    #[serde(skip_serializing)]
    last_render: Option<RenderOutput>,
    color_hash_enabled: bool,
}
impl Default for Scene {
    fn default() -> Self {
        Self::new()
    }
}
#[allow(unused)]
impl Scene {
    /// loads and return a new scene from a json / rscn file
    pub fn load_scene_from_file(path: PathBuf) -> anyhow::Result<Scene> {
        let mut directory_path = path.clone();
        directory_path.pop();
        let path_str = path.to_str().unwrap();
        info!("Scene: Loading new scene from {path_str}");
        let scene_and_path = parse_scene(path.clone());
        match scene_and_path {
            Ok(scene_and_path) => {
                let mut scene = scene_and_path.0;
                let mut paths = scene_and_path.1;
                let mut pathbuf = Vec::with_capacity(1);
                paths
                    .iter()
                    .for_each(|mut path| pathbuf.push(directory_path.join(path)));
                for i in pathbuf {
                    scene.load_object_from_file(i)?;
                }
                Ok(scene)
            }
            Err(error) => {
                error!("Scene: Importing Scene resulted in error: {error}");
                Err(error)
            }
        }
    }
    pub fn load_object_from_file(&mut self, path: PathBuf) -> Result<(), Error> {
        //! Adds new object from a obj file at path
        //! ## Parameter
        //! 'path': Path to the obj file
        //! ## Returns
        //! Result of either a reference to the new object or an error
        let path_str = path.to_str().unwrap();
        info!("Scene {self}: Loading object from {path_str}");
        let result = OBJParser::parse(path.clone());

        match result {
            Ok(objs) => {
                let mut material_name_list: Vec<String> = Vec::new();
                let mut material_list: Vec<Material> = Vec::new();

                if let Some(obj) = objs.material_path {
                    for i in obj {
                        let parsed = mtl_parser::MTLParser::parse(i.as_str());
                        match parsed {
                            Ok(parsed) => parsed.iter().for_each(|mat| {
                                material_list.push(Material::new(
                                    mat.name.clone(),
                                    mat.ka.iter().map(|a| *a as f64).collect(),
                                    mat.kd.iter().map(|a| *a as f64).collect(),
                                    mat.ks.iter().map(|a| *a as f64).collect(),
                                    mat.ns.into(),
                                    mat.d.into(),
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

                let mut tris = Vec::with_capacity(100);
                let mut material_index = Vec::with_capacity(10);
                for face in objs.faces {
                    let leng = face.v.len();
                    for i in 1..(leng - 1) {
                        let vs = (face.v[0], face.v[i], face.v[i + 1]);
                        tris.push(vs.0 as u32);
                        tris.push(vs.1 as u32);
                        tris.push(vs.2 as u32);
                        if let Some(m) = material_list
                            .iter()
                            .position(|x| x.name == face.material_name.clone())
                        {
                            material_index.push(m);
                        }
                    }
                }
                let mesh = Mesh::new(
                    objs.vertices,
                    tris,
                    Some(material_list),
                    Some(material_index),
                    Some(objs.name),
                    Some(path.to_string_lossy().to_string()),
                )?;
                info!("Scene {self}: Successfully loaded object from {path_str}");
                self.add_mesh(mesh);
                Ok(())
            }

            Err(error) => {
                error!("{self}: Parsing obj from {path_str} resulted in error: {error}");
                Err(error.into())
            }
        }
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
        let light = LightSource::new(
            Vec3::new(0.0, 0.0, 3.0),
            0.0,
            [1.0, 1.0, 1.0],
            "proto_light".to_owned(),
            Vec3::default(),
            LightType::Ambient,
        );
        self.add_sphere(sphere0);
        self.add_sphere(sphere1);
        self.add_sphere(sphere2);
        self.add_sphere(sphere3);
        self.add_sphere(sphere4);

        self.set_camera(cam);
        self.add_lightsource(light);
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
        //! A new scene with default values
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
        Self {
            scene_graph: SceneGraph::new(),
            // action_stack: ActionStack::new(),
            name: "scene".to_owned(),
            background_color: [1.0, 1.0, 1.0],
            render_engine: Option::from(Engine::new(
                RenderConfigBuilder::new()
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
                    .triangles_create(vec![])
                    .build(),
                RenderEngine::Raytracer,
            )),
            first_render: true,
            last_render: None,
            color_hash_enabled: true,
        } // todo: allow name and color as param
    }

    pub fn new_from_json(json_data: &str) -> Result<Scene, Error> {
        //! ## Paramter
        //! 'json_data': &str of a serialized scene
        //! ## Returns
        //! Result of new Scene from deserialized scene
        todo!("Deserialization of scene is not implemented")
        /* let deserialized =serde_json::from_str(json_data);
        match deserialized {
            Ok(scene) => {Ok(scene)},
            Err(_) => {Err(Error::msg("Failed to deserialize scene"))}
        } */
    }

    pub fn update_from_json(&mut self, json_data: &str) -> Result<(), Error> {
        //! ## Parameter
        //! 'json_data': &str of a serialized scene
        //! ## Returns
        //! Result of () or Error
        todo!("Deserialization of scene is not implemented")
        /* let deserialized = serde_json::from_str(json_data);
        match deserialized {
            Ok(scene) => {
                *self = scene;
                Ok(())
            },
            Err(_) => {Err(Error::msg("Failed to deserialize scene"))}
        } */
    }

    pub fn as_json(&self) -> Result<String, Error> {
        //! ## Returns:
        //! JSON serialization
        let s = serde_json::to_string(&self);
        match s {
            Ok(data) => Ok(data),
            Err(error) => Err(Error::msg(format!(
                "Error: Failed to serialize {self}: {error}"
            ))),
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
        info!("{self}: adding {:?}", mesh);
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
    pub fn get_meshes(&self) -> &Vec<Mesh> {
        //! ##  Returns
        //! a reference to a vector of all Meshes

        self.scene_graph.get_meshes()
    }

    pub fn get_light_sources(&self) -> &Vec<LightSource> {
        //! ## Returns
        //! Reference to a vector that holds all LightSources of the scene
        self.scene_graph.get_light_sources()
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

    pub fn set_last_render(&mut self, render: RenderOutput) {
        self.last_render = Some(render.clone());
        info!("{self}: Last render saved to buffer");
    }

    #[allow(dead_code)]
    pub fn export_render_img(&self, path: PathBuf) -> image::ImageResult<()> {
        let render = self.last_render.clone().ok_or_else(|| {
            image::ImageError::Parameter(image::error::ParameterError::from_kind(
                image::error::ParameterErrorKind::Generic("No render available".into()),
            ))
        })?;

        info!("{self}: Saved image to {:?}", path);
        export_img_png(path, render)
    }
}

impl std::fmt::Display for Scene {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Scene {}", self.get_name())
    }
}
