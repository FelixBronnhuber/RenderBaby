#[derive(Debug)]
pub struct Material {
    pub name: String,
    pub ambient_reflectivity: Vec<f64>,  //Ka
    pub diffuse_reflectivity: Vec<f64>,  //Kd
    pub specular_reflectivity: Vec<f64>, //Ks
    pub shininess: f64,                  //Ns
    pub transparency: f64,               //d
    pub texture_path: Option<String>,    //map_Kd
}
#[allow(dead_code)]
impl Material {
    pub fn new(
        name: String,
        ambient_reflectivity: Vec<f64>,
        diffuse_reflectivity: Vec<f64>,
        specular_reflectivity: Vec<f64>,
        shininess: f64,
        transparency: f64,
        texture_path: Option<String>,
    ) -> Self {
        //! Constructor for new material
        Material {
            name,
            ambient_reflectivity,
            diffuse_reflectivity,
            specular_reflectivity,
            shininess,
            transparency,
            texture_path,
        }
    }
}
impl Clone for Material {
    fn clone(&self) -> Material {
        Material {
            name: self.name.clone(),
            ambient_reflectivity: self.ambient_reflectivity.clone(),
            diffuse_reflectivity: self.diffuse_reflectivity.clone(),
            specular_reflectivity: self.specular_reflectivity.clone(),
            shininess: self.shininess,
            transparency: self.transparency,
            texture_path: self.texture_path.clone(),
        }
    }
}

impl Default for Material {
    fn default() -> Self {
        Material {
            name: String::new(),
            ambient_reflectivity: vec![0.0, 0.0, 0.0],
            diffuse_reflectivity: vec![0.0, 0.0, 0.0],
            specular_reflectivity: vec![0.0, 0.0, 0.0],
            shininess: 0.0,
            transparency: 0.0,
            texture_path: None,
        }
    }
}
