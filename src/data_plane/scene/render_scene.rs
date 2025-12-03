use anyhow::Error;
use engine_config::RenderConfigBuilder;
use glam::Vec3;
use scene_objects::{
    camera::Camera,
    geometric_object::GeometricObject,
    light_source::{LightSource, LightType},
    material::Material,
    sphere::Sphere,
    tri_geometry::TriGeometry,
};

use crate::{
    compute_plane::{engine::Engine, render_engine::RenderEngine},
    data_plane::{
        scene::{scene_graph::SceneGraph},
        scene_io::{obj_parser::parseobj, scene_parser::parse_scene},
    },
};
use crate::data_plane::scene_io::scene_parser::SceneParseError;

/// The scene holds all relevant objects, lightsources, camera
pub struct Scene {
    scene_graph: SceneGraph,
    //action_stack: ActionStack,
    background_color: [f32; 3],
    name: String,
    render_engine: Option<Engine>,
}
impl Default for Scene {
    fn default() -> Self {
        Self::new()
    }
}
#[allow(unused)]
impl Scene {
    /// loads and return a new scene from a json / rscn file
    pub fn load_scene_from_file(path: String) -> Result<Scene, SceneParseError> {
        parse_scene(path)
    }
    pub fn load_object_from_file(&mut self, path: String) -> Result<&TriGeometry, Error> {
        //! Adds new object from a obj file at path
        //! ## Parameter
        //! 'path': Path to the obj file
        //! ## Returns
        //! Result of either a reference to the new object or an error
        let objs = parseobj(path).unwrap();
        for obj in objs {
            self.add_object(Box::new(obj));
        }
        Ok(self
            .get_objects()
            .last()
            .unwrap()
            .as_ref()
            .as_any()
            .downcast_ref()
            .unwrap())
    }
    pub fn proto_init(&mut self) {
        //! For the early version: This function adds a sphere, a camera, and a lightsource
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

        let cam = Camera::new(Vec3::new(0.0, 0.0, 0.0), Vec3::default());
        let light = LightSource::new(
            Vec3::new(0.0, 0.0, 3.0),
            0.0,
            [1.0, 1.0, 1.0],
            "proto_light".to_owned(),
            Vec3::default(),
            LightType::Ambient,
        );
        self.add_object(Box::new(sphere0));
        self.add_object(Box::new(sphere1));
        self.add_object(Box::new(sphere2));
        self.add_object(Box::new(sphere3));
        self.add_object(Box::new(sphere4));

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
    /*pub fn set_camera_position(&mut self, pos: Vec<f32>) {
        self.get_camera().set_position(Vec3::new(pos[0], pos[1], pos[2]));
    }
    pub fn set_camera_rotation(&mut self, pitch: f32, yaw: f32) {
        self.get_camera().set_rotation(pitch, yaw);
    }*/
    pub fn new() -> Self {
        //! ## Returns
        //! A new scenen with default values
        Self {
            scene_graph: SceneGraph::new(),
            // action_stack: ActionStack::new(),
            name: "scene".to_owned(),
            background_color: [1.0, 1.0, 1.0],
            render_engine: Option::from(Engine::new(
                RenderConfigBuilder::new().build().unwrap(),
                RenderEngine::Raytracer,
            )),
        } // todo: allow name and color as param
    }

    pub fn add_object(&mut self, obj: Box<dyn GeometricObject>) {
        //! adds an object to the scene
        //! ## Arguments
        //! 'obj': GeometricObject that is to be added to the scene
        self.scene_graph.add_object(obj);
    }

    pub fn add_lightsource(&mut self, light: LightSource) {
        //! adds an LightSource to the scene
        //! ## Arguments
        //! 'light': LightSource that is to be added
        self.scene_graph.add_lightsource(light);
    }

    pub fn set_camera(&mut self, camera: Camera) {
        //! sets the scene camera to the passed camera
        //! ## Arguments
        //! 'camera': Camera that is to be the new scene camera
        self.scene_graph.set_camera(camera);
    }

    pub fn get_objects(&self) -> &Vec<Box<dyn GeometricObject>> {
        //! ##  Returns
        //! a reference to a vector of all render objects

        self.scene_graph.get_objects()
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
        self.render_engine = Some(engine);
    }

    pub fn get_name(&self) -> &String {
        //!## Returns
        //! Reference to the scene name
        &self.name
    }

    pub fn set_name(&mut self, name: String) {
        //! ## Arguments
        //! 'name' : new scene name
        self.name = name;
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
    }

    #[allow(dead_code)]
    pub fn export_render_img(&self, path: String) -> Result<(), Error> {
        todo!()
    }
}
