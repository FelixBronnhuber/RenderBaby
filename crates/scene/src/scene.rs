use anyhow::Error;
use engine_main::Engine;
use glam::Vec3;

use crate::{
    action_stack::ActionStack, geometric_object::{
        Camera, GeometricObject, LightSource, Material, Rotation, Sphere, TriGeometry, Triangle,
    }, obj_parser::parseobj, scene_graph::SceneGraph
};

/// The scene holds all relevant objects, lightsources, camera ...
pub struct Scene {
    /*objects: Vec<GeometricObject>,
    camera: Camera,
    light_sources: Vec<LightSource>
    */
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
    /*pub fn image_buffer(&self) -> Vec<u8> {
        todo!()
        // render engine uses Vec<u8>, with 4 entries beeing one pixel. We might transform this to something else?
    }*/
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
        let obj = parseobj(path);
        self.add_object(Box::new(obj));
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
        let color = [0.0, 1.0, 0.0];
        let sphere = Sphere::new(Vec3::new(2.0, 0.0, 0.0), 1.0, Material::default(), color);
        let cam = Camera::new(Vec3::new(2.0, 0.0, 0.0), Rotation::new(0.0, 0.0));
        let light = LightSource::new(
            Vec3::new(0.0, 0.0, 3.0),
            0.0,
            [1.0, 1.0, 1.0],
            "proto_light".to_owned(),
        );
        self.add_object(Box::new(sphere));
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
            render_engine: None,
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
    pub fn get_render_engine(&self) -> &Option<Engine> {
        &self.render_engine
    }
    pub fn get_render_engine_mut(&mut self) -> &mut Option<Engine> {
        &mut self.render_engine
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
    pub fn set_backgroubd_color(&mut self, color: [f32; 3]) {
        self.background_color = color;
    }
}

/* pub struct SceneConfig{
    background_color: [f32; 3]
} */
