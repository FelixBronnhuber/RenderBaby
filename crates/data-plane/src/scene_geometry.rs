use std::fmt::Debug;

pub trait SceneGeometry : Debug {
    /// Trait for every Part of the scene with a Geometry
    // what does SceneGeometry need
    fn geom(&self) -> dyn Geom; // todo needs return type
    fn apply_scene_action(&self);
    fn color(&self) -> &str; // todo what is the type? should it be called material / texture...
    //fn new(&self, geom: dyn Geom, color: &str) -> Self;
}

pub struct Sphere<'a> {
    /// Represents a Sphere ...
    geom: SphereGeom<'a>,
    color: String
}

impl SceneGeometry for Sphere {
    fn geom(&self) -> SphereGeom {
        self.geom
    }

    fn apply_scene_action(&self) {
        todo!()
    }

    fn color(&self) -> String {
        self.color
    }

}

impl <'a> Sphere<'a> {
    fn new(&self, geom:SphereGeom<'a> , color: String) -> Self {
        Sphere{geom, color}
    }
}

pub struct Vec3d {
    /// For now: Helper for 3d vec
    x: f64,
    y: f64,
    z: f64,
}

pub trait Geom {
    fn scale(&mut self, factor: f64);
    fn translate(&mut self, vec: Vec3d);
}
pub struct SphereGeom<'a> {
    center: &'a Vec3d,
    radius: f64,
}
impl <'a> Geom for SphereGeom<'a>{
    fn scale(&mut self, factor: f64) {
        self.radius *= factor;
    }

    fn translate(&mut self, vec: Vec3d) {
        todo!() // todo Vec3d needs add ...
    }
}


