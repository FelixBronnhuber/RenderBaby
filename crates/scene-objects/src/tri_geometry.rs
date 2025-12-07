use glam::Vec3;
use serde::Serialize;

use crate::{
    geometric_object::{GeometricObject, SceneObject},
    material::Material,
};

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize)]
pub struct TriGeometry {
    #[serde(skip_serializing)]
    triangles: Vec<Triangle>,
    name: String,
    #[serde(skip_serializing)]
    material: Material,
    path: Option<String>,
    scale: Vec3,
    #[serde(skip_serializing)]
    translation: Vec3,
    rotation: Vec3,
    #[serde(rename(serialize = "position"))]
    a_position: Option<Vec3>,
}
impl GeometricObject for TriGeometry {
    fn scale(&mut self, factor: f32) {
        for tri in self.get_triangles_mut() {
            tri.scale(factor);
        }
    }

    fn translate(&mut self, vec: Vec3) {
        for tri in self.get_triangles_mut() {
            tri.translate(vec);
        }
    }

    fn rotate(&mut self, _vec: Vec3) {
        todo!()
    }
}
impl SceneObject for TriGeometry {
    fn get_path(&self) -> Option<&str> {
        todo!()
    }

    fn get_scale(&self) -> Vec3 {
        todo!()
    }

    fn get_translation(&self) -> Vec3 {
        todo!()
    }

    fn get_rotation(&self) -> Vec3 {
        todo!()
    }
}
#[allow(dead_code)]
impl TriGeometry {
    pub fn get_triangles_mut(&mut self) -> &mut Vec<Triangle> {
        &mut self.triangles
    }
    pub fn get_triangles(&self) -> &Vec<Triangle> {
        &self.triangles
    }
    pub fn get_name(&self) -> String {
        self.name.clone()
    }
    pub fn set_material(&mut self, material: Material) {
        self.material = material;
    }
    pub fn new(triangles: Vec<Triangle>) -> Self {
        let mut a_point: Option<Vec3> = None;
        if let Some(t) = triangles.first() {
            a_point = t.get_points().first().copied();
        }
        TriGeometry {
            triangles,
            scale: Vec3::new(1.0, 1.0, 1.0),
            translation: Vec3::default(),
            rotation: Vec3::default(),
            path: None,
            name: "unnamed".to_owned(),
            material: Material::default(),
            a_position: a_point,
        }
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Triangle {
    points: Vec<Vec3>,
    material: Option<Material>,
}
#[allow(dead_code)]
impl Triangle {
    pub fn new(points: Vec<Vec3>, material: Option<Material>) -> Self {
        Triangle { points, material }
    }
    pub fn get_points(&self) -> &Vec<Vec3> {
        &self.points
    }
    /*pub fn add_point(&mut self, point: Vec3) {
        self.points.push(point);
    }*/
    pub fn get_material(&self) -> &Option<Material> {
        &self.material
    }
    pub fn set_material(&mut self, material: Option<Material>) {
        self.material = material;
    }
    pub fn set_points(&mut self, points: Vec<Vec3>) {
        self.points = points;
        // todo: check for points length, otherwise error
    }
    pub fn add_point(&mut self, point: Vec3) {
        self.points.push(point);
    }
}
impl GeometricObject for Triangle {
    fn translate(&mut self, vec: Vec3) /* -> &Vec<Vec3>*/
    {
        for point in &mut self.points {
            *point += vec;
        }
        //self.get_points()
    }

    fn scale(&mut self, _factor: f32) {
        todo!()
    }

    fn rotate(&mut self, _vec: Vec3) {
        todo!()
    }
}
