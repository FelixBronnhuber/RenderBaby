use std::sync::Arc;

use engine_wgpu_wrapper::{GpuDevice, RenderState};
use glam::Vec3;

use crate::{
    action_stack::ActionStack,
    geometric_object::{Camera, GeometricObject, LightSource, Material, Rotation, Sphere},
    scene_graph::SceneGraph,
};

/// The scene holds all relevant objects, lightsources, camera ...
pub struct Scene {
    /*objects: Vec<GeometricObject>,
    camera: Camera,
    light_sources: Vec<LightSource>
    */
    scene_graph: SceneGraph,
    action_stack: ActionStack,
}
impl Scene {
    pub fn image_buffer(&self) -> Vec<u8> {
        todo!()
        // render engine uses Vec<u8>, with 4 entries beeing one pixel. We might transform this to something else?
    }

    pub fn proto_init(&mut self) {
        //! For the early version: This function adds a sphere, a camera, and a lightsource
        let color = [0, 128, 0];
        let sphere = Sphere::new(Vec3::new(2.0, 0.0, 0.0), 1.0, Material {}, color);
        let cam = Camera::new(Vec3::new(2.0, 0.0, 0.0), Rotation::new(0.0, 0.0));
        let light = LightSource::new(Vec3::new(0.0, 0.0, 3.0), 0.0);
        self.add_object(Box::new(sphere));
        self.set_camera(cam);
        self.add_lightsource(light);
    }
    pub fn get_camera(&mut self) -> &mut Camera {
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
        }
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

    //action stack fns: currently empty
    pub fn undo(&mut self) {
        self.action_stack.undo();
    }
    pub fn redo(&mut self) {
        self.action_stack.redo();
    }
}
