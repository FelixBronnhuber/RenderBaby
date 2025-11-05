use glam::Vec3;

use crate::{action_stack::ActionStack, geometric_object::{Camera, GeometricObject, LightSource, Rotation}, scene_graph::SceneGraph};


pub struct Scene{
    /*objects: Vec<GeometricObject>,
    camera: Camera,
    light_sources: Vec<LightSource>
    */
    scene_graph: SceneGraph,
    action_stack: ActionStack
}
impl Scene {
    pub fn get_camera(&mut self) -> &mut Camera {
        self.scene_graph.get_camera()
    }
    pub fn set_camera_position(&mut self, pos: Vec<f32>) {
        self.get_camera().set_position(Vec3::new(pos[0], pos[1], pos[2]));
    }
    pub fn set_camera_rotation(&mut self, pitch: f32, yaw: f32) {
        self.get_camera().set_rotation(pitch, yaw);
    }
    pub fn new() -> Self {
        Self {scene_graph: SceneGraph::new(), action_stack: ActionStack::new()}
    }
}

