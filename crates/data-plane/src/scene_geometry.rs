use std::fmt::Debug;

pub trait SceneGeometry : Debug {
    /// Trait for every Part of the scene with a Geometry
    // what does SceneGeometry need
    fn geom(&self) -> dyn Geom; // todo needs return type
    fn apply_scene_action(&self);
    fn color(&self) -> &str; // todo what is the type? should it be called material / texture...
    fn name(&self) -> &str;
    //fn new(&self, geom: dyn Geom, color: &str) -> Self;
}

pub struct Sphere<'a> {
    /// Represents a Sphere ...
    geom: SphereGeom<'a>,
    color: String,
    name: String
}
impl <'a> std::fmt::Debug for Sphere<'a>{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: Test sphere", self.name())
        //todo!()
    }
}
impl <'a> SceneGeometry for Sphere<'a> {
    fn geom(&self) -> &SphereGeom {
        &self.geom
    }

    fn apply_scene_action(&self) {
        todo!()
    }

    fn color(&self) -> &str {
        &self.color
    }
    
    fn name(&self) -> &str {
        self.name.as_str()
    }

}

impl <'a> Sphere<'a> {
    pub fn new(geom:SphereGeom<'a> , color: String, name: String) -> Self {
        Sphere{geom, color, name}
    }
}

pub struct Vec3d {
    /// For now: Helper for 3d vec
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub trait Geom {
    fn scale(&mut self, factor: f64);
    fn translate(&mut self, vec: Vec3d);
}
pub struct SphereGeom<'a> {
    center: &'a Vec3d,
    radius: f64,
}
impl<'a> SphereGeom<'a> {
    pub fn new(center: &Vec3d, radius: f64) -> Self {
        SphereGeom {center, radius}
    }
}
impl <'a> Geom for SphereGeom<'a>{
    fn scale(&mut self, factor: f64) {
        self.radius *= factor;
    }

    fn translate(&mut self, vec: Vec3d) {
        todo!() // todo Vec3d needs add ...
    }
}


