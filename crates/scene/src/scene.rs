use crate::{
    action_stack::ActionStack,
    geometric_object::{
        Camera, GeometricObject, LightSource, LightType, Material, Rotation, Sphere, TriGeometry,
        Triangle,
    },
    obj_parser::parseobj,
    scene_graph::SceneGraph,
    scene_parser::parse_scene,
};
use anyhow::Error;
use engine_config::RenderConfigBuilder;
use engine_main::{Engine, RenderEngine};
use glam::Vec3;

/// The scene holds all relevant objects, lightsources, camera ...
pub struct Scene {
    scene_graph: SceneGraph,
    action_stack: ActionStack,
    //render_engine: Engine::new(),
    background_color: [f32; 3],
    name: String,
    render_engine: Option<Engine>,
}
impl Default for Scene {
    fn default() -> Self {
        Self::new()
    }
}
impl Scene {
    pub fn load_scene_from_file(path: String) -> Scene {
        parse_scene(path)
    }
    pub fn load_object_from_file(&mut self, path: String) -> Result<&TriGeometry, Error> {
        //! loads object from file. Adds object to scene and returns object if successfull
        /* let p0 = Vec3::new(0.0, 0.0, 0.0);
        let p1 = Vec3::new(1.0, 0.0, 0.0);
        let p2 = Vec3::new(0.0, 1.0, 0.0);
        let p2 = Vec3::new(0.0, 0.0, 1.0);
        // todo: color?
        let t0 = Triangle::new(vec![], None);
        let t1 = Triangle::new(vec![], None);
        let t2 = Triangle::new(vec![], None);
        let t3 = Triangle::new(vec![], None);
        let obj = TriGeometry::new(vec![t0, t1, t2, t3], Material::default()); */
        let objs = parseobj(path).unwrap();
        for obj in objs {
            self.add_object(Box::new(obj));
        }

        //Ok(&res)
        //todo: this is very ugly
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

        let p0 = Vec3::new(0.0, 0.0, 1.0);
        let p1 = Vec3::new(1.0, 0.0, 1.0);
        let p2 = Vec3::new(1.0, 1.0, 1.0);
        let p3 = Vec3::new(0.0, 1.0, 1.0);
        let t0 = Triangle::new(vec![p0, p1, p2], None);
        let t1 = Triangle::new(vec![p0, p2, p3], None);
        let tri = TriGeometry::new(vec![t0, t1]);
        let cam = Camera::new(Vec3::new(0.0, 0.0, 0.0), Rotation::new(0.0, 0.0));
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

        self.add_object(Box::new(tri));

        self.set_camera(cam);
        self.add_lightsource(light);
    }

    pub fn get_camera_mut(&mut self) -> &mut Camera {
        self.scene_graph.get_camera_mut()
    }
    pub fn get_camera(&self) -> &Camera {
        self.scene_graph.get_camera()
    }
    /*pub fn set_camera_position(&mut self, pos: Vec<f32>) {
        self.get_camera().set_position(Vec3::new(pos[0], pos[1], pos[2]));
    }
    pub fn set_camera_rotation(&mut self, pitch: f32, yaw: f32) {
        self.get_camera().set_rotation(pitch, yaw);
    }*/
    pub fn new() -> Self {
        Self {
            scene_graph: SceneGraph::new(),
            action_stack: ActionStack::new(),
            name: "scene".to_owned(),
            background_color: [1.0, 1.0, 1.0],
            render_engine: Option::from(Engine::new(
                RenderConfigBuilder::new().build().unwrap(),
                RenderEngine::Raytracer,
            )),
        } // todo: allow name and color as param
    }

    pub fn add_object(&mut self, obj: Box<dyn GeometricObject>) {
        self.scene_graph.add_object(obj);
    }
    pub fn add_lightsource(&mut self, light: LightSource) {
        self.scene_graph.add_lightsource(light);
    }
    pub fn set_camera(&mut self, camera: Camera) {
        self.scene_graph.set_camera(camera);
    }

    pub fn get_objects(&self) -> &Vec<Box<dyn GeometricObject>> {
        self.scene_graph.get_objects()
    }
    pub fn get_light_sources(&self) -> &Vec<LightSource> {
        self.scene_graph.get_light_sources()
    }
    pub fn get_render_engine(&self) -> &Engine {
        self.render_engine.as_ref().expect("No render engine found")
    }
    pub fn get_render_engine_mut(&mut self) -> &mut Engine {
        self.render_engine.as_mut().expect("No render engine found")
    }
    pub fn set_render_engine(&mut self, engine: Engine) {
        self.render_engine = Some(engine);
    }

    //action stack fns: currently empty
    pub fn undo(&mut self) {
        self.action_stack.undo();
    }
    pub fn redo(&mut self) {
        self.action_stack.redo();
    }
    pub fn get_name(&self) -> &String {
        &self.name
    }
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
    pub fn get_background_color(&self) -> [f32; 3] {
        self.background_color
    }
    pub fn set_background_color(&mut self, color: [f32; 3]) {
        self.background_color = color;
    }
}
