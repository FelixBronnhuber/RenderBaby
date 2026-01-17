use glam::Vec3;
use std::path::PathBuf;

#[allow(dead_code)]
///Defines some basic geometric functions that all Render Objects should offer
pub trait GeometricObject {
    fn scale(&mut self, factor: Vec3); // TODO: scale 3d?
    fn translate(&mut self, vec: Vec3);
    fn rotate(&mut self, vec: Vec3);
}

#[allow(dead_code)]
/// Defines functions that define how the object has changed from its base
pub trait SceneObject: GeometricObject {
    fn get_path(&self) -> Option<PathBuf>;
    //fn set_path(&mut self, path: String);
    fn get_scale(&self) -> Vec3;
    fn get_translation(&self) -> Vec3;
    fn get_rotation(&self) -> Vec3;
    // todo color?
}
