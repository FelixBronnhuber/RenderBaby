use std::path::PathBuf;
use anyhow::Error;
use engine_config::{RenderConfigBuilder, Uniforms, RenderOutput};
use glam::Vec3;
use log::{info, error};
use scene_objects::{
    camera::{Camera, Resolution},
    light_source::{LightSource, LightType},
    material::Material,
    mesh::Mesh,
    sphere::Sphere,
    tri_geometry::TriGeometry,
};
use scene_objects::tri_geometry::Triangle;
use serde::Serialize;

use crate::{
    compute_plane::{engine::Engine, render_engine::RenderEngine},
    data_plane::{
        scene::scene_graph::SceneGraph,
        scene_io::{img_export::export_img_png, obj_parser::OBJParser, scene_parser::parse_scene},
        scene_proxy::proxy_scene::ProxyScene,
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
    pub proxy_scene: ProxyScene,
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
        let path_str = path.to_str().unwrap();
        info!("Scene: Loading new scene from {path_str}");
        parse_scene(path)
    }
    pub fn load_object_from_file(&mut self, path: PathBuf) -> Result<TriGeometry, Error> {
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
                                info!("{self}: mtl file at {i} could not be parsed");
                            }
                        }
                    }

                    material_list
                        .iter()
                        .for_each(|mat| material_name_list.push(mat.name.clone()));
                }
                let mut trianglevector: Vec<Triangle> = Vec::with_capacity(100);
                objs.faces.len();
                for face in objs.faces {
                    let leng = face.v.len();
                    for i in 1..(leng - 1) {
                        let vs = (face.v[0], face.v[i], face.v[i + 1]);
                        let triangle = vec![
                            Vec3::new(
                                objs.vertices[((vs.0 - 1.0) * 3.0) as usize],
                                objs.vertices[(((vs.0 - 1.0) * 3.0) + 1.0) as usize],
                                objs.vertices[(((vs.0 - 1.0) * 3.0) + 2.0) as usize],
                            ),
                            Vec3::new(
                                objs.vertices[((vs.1 - 1.0) * 3.0) as usize],
                                objs.vertices[(((vs.1 - 1.0) * 3.0) + 1.0) as usize],
                                objs.vertices[(((vs.1 - 1.0) * 3.0) + 2.0) as usize],
                            ),
                            Vec3::new(
                                objs.vertices[((vs.2 - 1.0) * 3.0) as usize],
                                objs.vertices[(((vs.2 - 1.0) * 3.0) + 1.0) as usize],
                                objs.vertices[(((vs.2 - 1.0) * 3.0) + 2.0) as usize],
                            ),
                        ];

                        let material_name = face.material_name.clone();
                        if let Some(material) =
                            material_list.iter().find(|a| a.name == material_name)
                        {
                            trianglevector.push(Triangle::new(triangle, Some(material.clone())));
                        } else {
                            trianglevector.push(Triangle::new(triangle, None));
                        };
                    }
                }
                let mut tri = TriGeometry::new(trianglevector);
                tri.set_name(objs.name);
                self.add_tri_geometry(tri.clone());
                return Result::Ok(tri);
            }
            Err(error) => {
                error!("{self}: Parsing obj from {path_str} resulted in error: {error}");
                return Err(error.into());
            }
        }
        self.update_proxy();
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
        let mut res = Self {
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
            proxy_scene: ProxyScene::default(),
        };
        res.update_proxy();
        res
    }

    pub fn add_tri_geometry(&mut self, tri: TriGeometry) {
        //! adds an object to the scene
        //! ## Arguments
        //! 'tri': TriGeometry that is to be added to the scene
        //!
        info!("{self}: adding TriGeometry {:?}", tri.get_name());
        self.scene_graph.add_tri_geometry(tri);
        self.update_proxy();
    }
    pub fn add_sphere(&mut self, sphere: Sphere) {
        //! adds an object to the scene
        //! ## Arguments
        //! 'sphere': GeometricObject that is to be added to the scene
        info!("{self}: adding {:?}", sphere);
        self.scene_graph.add_sphere(sphere);
        self.update_proxy();
    }
    pub fn add_mesh(&mut self, mesh: Mesh) {
        //! adds an object to the scene
        //! ## Arguments
        //! 'mesh': GeometricObject that is to be added to the scene
        info!("{self}: adding {:?}", mesh);
        self.scene_graph.add_mesh(mesh);
        self.update_proxy();
    }

    pub fn add_lightsource(&mut self, light: LightSource) {
        //! adds an LightSource to the scene
        //! ## Arguments
        //! 'light': LightSource that is to be added
        info!("{self}: adding LightSource {light}");
        self.scene_graph.add_lightsource(light);
        self.update_proxy();
    }

    pub fn clear_spheres(&mut self) {
        self.scene_graph.clear_spheres();
        self.update_proxy();
    }

    pub fn clear_polygons(&mut self) {
        self.scene_graph.clear_tri_geometries();
        self.update_proxy();
    }
    pub fn set_camera(&mut self, camera: Camera) {
        //! sets the scene camera to the passed camera
        //! ## Arguments
        //! 'camera': Camera that is to be the new scene camera
        info!("{self}: set camera to {camera}");
        self.scene_graph.set_camera(camera);
    }

    pub fn get_tri_geometries(&self) -> &Vec<TriGeometry> {
        //! ##  Returns
        //! a reference to a vector of all TriGeometries

        self.scene_graph.get_tri_geometries()
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

    //proxy updates
    pub(crate) fn update_proxy(&mut self) -> Result<(), Error> {
        self.proxy_scene = ProxyScene::new_from_real_scene(self);
        Ok(())
    }
    pub(crate) fn update_from_proxy(&mut self) -> Result<(), Error> {
        self.update_real_name()?;
        self.update_real_objects()?;
        self.update_real_camera()?;
        self.update_real_lights()?;
        self.update_real_background_color()?;
        self.update_real_misc()?;
        Ok(())
    }

    fn update_real_name(&mut self) -> Result<(), Error> {
        if &self.proxy_scene.scene_name != self.get_name() {
            info!("{}:Changing name to  {}", self, self.proxy_scene.scene_name);
            self.name = self.proxy_scene.scene_name.clone();
        }
        Ok(())
    }

    fn update_real_background_color(&mut self) -> Result<(), Error> {
        let c0 = &self.proxy_scene.background_color;
        let c1 = self.get_background_color();

        if *c0 == c1 {
            info!(
                "{}: Changing Background color to {:?}",
                self, self.proxy_scene.background_color
            );
            self.background_color = [c0.r, c0.g, c0.b];
        }
        Ok(())
    }
    fn update_real_objects(&mut self) -> Result<(), Error> {
        let mut real_objects = self.get_tri_geometries();
        let proxy_objects = &self.proxy_scene.objects;
        for i in 0..real_objects.len() {}
        todo!()
    }
    fn update_real_camera(&mut self) -> Result<(), Error> {
        if self.proxy_scene.camera == *self.get_camera() {
            self.set_camera(self.proxy_scene.camera.clone().into());
        }
        Ok(())
    }
    fn update_real_lights(&mut self) -> Result<(), Error> {
        todo!()
    }
    fn update_real_misc(&mut self) -> Result<(), Error> {
        todo!()
        //Ok(())
    }
}

impl std::fmt::Display for Scene {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Scene {}", self.get_name())
    }
}
