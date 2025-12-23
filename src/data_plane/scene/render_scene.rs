use std::path::{PathBuf};
use anyhow::{Error};
use engine_config::{RenderConfigBuilder, Uniforms, RenderOutput};
use glam::Vec3;
use log::{debug, error, info};
use scene_objects::{
    camera::{Camera, Resolution},
    light_source::{LightSource, LightType},
    material::Material,
    mesh::Mesh,
    sphere::Sphere,
};
use crate::{
    compute_plane::{engine::Engine, render_engine::RenderEngine},
    data_plane::{
        scene::{
            scene_engine_adapter::{
                camera_to_render_uniforms, mesh_to_render_data, sphere_to_render_sphere,
            },
            scene_graph::SceneGraph,
        },
        scene_io::{
            img_export::export_img_png, obj_parser::OBJParser, scene_importer::parse_scene,
        },
    },
};
use crate::data_plane::scene_io::mtl_parser;

type RenderSphere = engine_config::Sphere;
type RenderUniforms = engine_config::Uniforms;
type RenderCamera = engine_config::Camera;

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
                        tris.push(vs.0 as u32 - 1);
                        tris.push(vs.1 as u32 - 1);
                        tris.push(vs.2 as u32 - 1);
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
        let rotation = RenderCamera::default().dir; //Engine uses currently a direction vector
        let pane_width = RenderCamera::default().pane_width;
        let render_camera = RenderCamera::new(
            cam.get_fov(),
            pane_width,
            [position.x, position.y, position.z],
            rotation,
        );
        let mut res = Self {
            scene_graph: SceneGraph::new(),
            // action_stack: ActionStack::new(),
            name: "scene".to_owned(),
            background_color: [1.0, 1.0, 1.0],
            render_engine: None,
            render_config_builder: RenderConfigBuilder::new(),
            first_render: true,
            last_render: None,
            color_hash_enabled: true,
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
            .triangles_create(vec![]);

        res.set_render_engine(Engine::new(
            res.render_config_builder.clone().build(),
            RenderEngine::Raytracer,
        ));
        res
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

    pub fn export_render_img(&self, path: PathBuf) -> image::ImageResult<()> {
        let render = self.last_render.clone().ok_or_else(|| {
            image::ImageError::Parameter(image::error::ParameterError::from_kind(
                image::error::ParameterErrorKind::Generic("No render available".into()),
            ))
        })?;

        info!("{self}: Saved image to {:?}", path);
        export_img_png(path, render)
    }

    fn get_render_spheres(&self) -> Vec<RenderSphere> {
        //! ## Returns
        //! a Vec that contains all Scene spheres as engine_config::Sphere
        self.get_spheres()
            .iter()
            .map(sphere_to_render_sphere)
            .collect()
    }
    pub(crate) fn get_render_uniforms(
        &self,
        spheres_count: u32,
        triangles_count: u32,
    ) -> RenderUniforms {
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

    fn get_render_tris(&self) -> Vec<(Vec<f32>, Vec<u32>)> {
        //! ## Returns
        //! Vector of touples, with each of the touples representing a TriGeometry defined by the points and the triangles build from the points.
        self.get_meshes().iter().map(mesh_to_render_data).collect()
    }

    pub fn render(&mut self) -> Result<RenderOutput, Error> {
        //! calls the render engine for the scene self.
        //! ## Returns
        //! Result of either the RenderOutput or a error
        info!("{self}: Render has been called. Collecting render parameters");

        let render_spheres = self.get_render_spheres();
        let render_tris = self.get_render_tris();
        debug!("Scene mesh data: {:?}", self.get_meshes());
        debug!("Collected mesh data: {:?}", render_tris);

        let spheres_count = render_spheres.len() as u32;
        let triangles_count = render_tris
            .iter()
            .map(|(_, tri)| tri.len() as u32 / 3)
            .sum();

        let uniforms = self.get_render_uniforms(spheres_count, triangles_count);

        // Collect all vertices and triangles into flat vectors
        let (all_vertices, all_triangles) = if render_tris.is_empty() {
            (vec![], vec![])
        } else {
            let mut all_verts = vec![];
            let mut all_tris = vec![];
            let mut vertex_offset = 0u32;

            for (verts, tris) in render_tris {
                let vertex_count = (verts.len() / 3) as u32;

                for tri_idx in tris {
                    all_tris.push(tri_idx + vertex_offset);
                }

                all_verts.extend(verts);

                vertex_offset += vertex_count;
            }
            (all_verts, all_tris)
        };
        info!("Collected vertices: {:?}", all_vertices);
        info!("Collected tris: {:?}", all_triangles);
        info!(
            "{self}: Collected render parameter: {} spheres, {} triangles consisting of {} vertices. Building render config",
            render_spheres.len(),
            triangles_count,
            all_vertices.len() / 3
        );

        self.render_config_builder = if self.get_first_render() {
            self.set_first_render(false);
            // NOTE: *_create is for the first initial render which initializes all the buffers etc.
            RenderConfigBuilder::new()
                .uniforms_create(uniforms)
                .spheres_create(render_spheres)
                .vertices_create(all_vertices)
                .triangles_create(all_triangles)
        } else {
            // NOTE: * otherwise the values are updated with the new value an the unchanged fields
            // are kept as is. See: ../../../crates/engine-config/src/render_config.rs - `Change<T>`
            RenderConfigBuilder::new()
                .uniforms(uniforms)
                .spheres(render_spheres)
                .vertices(all_vertices)
                .triangles(all_triangles)
        };
        let rc = self.render_config_builder.clone().build();

        let engine = self.get_render_engine_mut();

        let output = engine.render(rc);
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
                error!("{self}: The following error occurred when rendering: {error}");
                Err(error)
            }
        }
    }
}

impl std::fmt::Display for Scene {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Scene {}", self.get_name())
    }
}
