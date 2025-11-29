#[derive(Debug)]
pub struct Material {
    ambient_reflectivity: Vec<f64>,  //Ka
    diffuse_reflectivity: Vec<f64>,  //Kd
    specular_reflectivity: Vec<f64>, //Ks
    shininess: f64,                  //Ns
    transparency: f64,               //d
}
#[allow(dead_code)]
impl Material {
    pub fn new(
        ambient_reflectivity: Vec<f64>,
        diffuse_reflectivity: Vec<f64>,
        specular_reflectivity: Vec<f64>,
        shininess: f64,
        transparency: f64,
    ) -> Self {
        Material {
            ambient_reflectivity,
            diffuse_reflectivity,
            specular_reflectivity,
            shininess,
            transparency,
        }
    }
}
impl Clone for Material {
    fn clone(&self) -> Material {
        Material {
            ambient_reflectivity: self.ambient_reflectivity.clone(),
            diffuse_reflectivity: self.diffuse_reflectivity.clone(),
            specular_reflectivity: self.specular_reflectivity.clone(),
            shininess: self.shininess,
            transparency: self.transparency,
        }
    }
}

impl Default for Material {
    fn default() -> Self {
        Material {
            ambient_reflectivity: vec![0.0, 0.0, 0.0],
            diffuse_reflectivity: vec![0.0, 0.0, 0.0],
            specular_reflectivity: vec![0.0, 0.0, 0.0],
            shininess: 0.0,
            transparency: 0.0,
        }
    }
}
