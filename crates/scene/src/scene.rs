use crate::{
    action_stack::ActionStack,
    geometric_object::{
        Camera, GeometricObject, LightSource, Material, Rotation, Sphere, TriGeometry, Triangle,
    },
    scene_graph::SceneGraph,
};
use anyhow::Error;
use engine_config::RenderConfigBuilder;
use engine_main::{Engine, RenderEngine};
use glam::Vec3;

/// The scene holds all relevant objects, lightsources, camera ...
pub struct Scene {
    scene_graph: SceneGraph,
    action_stack: ActionStack,
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
    /*pub fn image_buffer(&self) -> Vec<u8> {
        todo!()
        // render engine uses Vec<u8>, with 4 entries beeing one pixel. We might transform this to something else?
    }*/
    pub fn load_object_from_file(&mut self, path: String) -> Result<&TriGeometry, Error> {
        //! loads object from file. Adds object to scene and returns object if successfull
        //! Currently place holder!
        let p0 = Vec3::new(0.0, 0.0, 0.0);
        let p1 = Vec3::new(1.0, 0.0, 0.0);
        let p2 = Vec3::new(0.0, 1.0, 0.0);
        let p2 = Vec3::new(0.0, 0.0, 1.0);
        // todo: color?
        let t0 = Triangle::new(vec![], None);
        let t1 = Triangle::new(vec![], None);
        let t2 = Triangle::new(vec![], None);
        let t3 = Triangle::new(vec![], None);
        let res = TriGeometry::new(vec![t0, t1, t2, t3]);
        self.add_object(Box::new(res));
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

        let sphere0 = Sphere::new(Vec3::new(0.0, 0.6, 1.0), 0.5, Material {}, magenta);
        let sphere1 = Sphere::new(Vec3::new(-0.6, 0.0, 1.0), 0.5, Material {}, green);
        let sphere2 = Sphere::new(Vec3::new(0.0, 0.0, 1.0), 0.5, Material {}, red);
        let sphere3 = Sphere::new(Vec3::new(0.6, 0.0, 1.0), 0.5, Material {}, blue);
        let sphere4 = Sphere::new(Vec3::new(0.0, -0.6, 1.0), 0.5, Material {}, cyan);

        let p0 = Vec3::new(0.0, 0.0, 0.0);
        let p1 = Vec3::new(1.0, 0.0, 0.0);
        let p2 = Vec3::new(0.0, 1.0, 0.0);
        let p3 = Vec3::new(0.0, 0.0, 1.0);
        let t0 = Triangle::new(vec![p0, p1, p2], None);
        let t1 = Triangle::new(vec![p1, p2, p3], None);
        let t2 = Triangle::new(vec![p1, p3, p0], None);
        let t3 = Triangle::new(vec![p0, p2, p3], None);
        let tri = TriGeometry::new(vec![t0, t1, t2, t3]);
        let cam = Camera::new(Vec3::new(2.0, 0.0, 0.0), Rotation::new(0.0, 0.0));
        let light = LightSource::new(
            Vec3::new(0.0, 0.0, 3.0),
            0.0,
            [1.0, 1.0, 1.0],
            "proto_light".to_owned(),
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

/* pub struct SceneConfig{
    background_color: [f32; 3]
} */
