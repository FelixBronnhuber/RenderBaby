use glam::Vec3;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct SceneFile {
    pub scene_name: String,
    pub objects: Vec<ParsingObject>,
    pub lights: Vec<FileLightSource>,
    pub camera: FileCamera,
    pub background_color: FileColor,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spheres: Option<Vec<FileSphere>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ray_samples: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hash_color: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ParsingObject {
    pub name: String,
    pub path: String,
    pub scale: Vec3d,
    pub translation: Vec3d,
    pub rotation: Vec3d, //x = roll, y = pitch, z = yaw
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FileLightSource {
    pub name: String,
    pub r#type: String,
    pub position: Vec3d,
    pub luminosity: f32,
    pub color: FileColor,
    pub rotation: Option<Vec3d>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FileCamera {
    pub position: Vec3d,
    pub look_at: Vec3d,
    pub up: Vec3d, //roll
    pub pane_distance: f32,
    pub pane_width: f32,
    pub resolution: Resolution,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FileColor {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a: Option<f32>, //ungenutzt
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Vec3d {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Resolution {
    pub x: u32,
    pub y: u32,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct FileSphere {
    pub center: Vec3,
    pub radius: f32,
    pub material: FileMaterial,
    pub color: FileColor,
    pub name: String,
    pub scale: Vec3,
    pub translation: Vec3,
    pub rotation: Vec3,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FileMaterial {
    pub name: String,
    pub ambient_reflectivity: Vec<f64>,  //Ka
    pub diffuse_reflectivity: Vec<f64>,  //Kd
    pub specular_reflectivity: Vec<f64>, //Ks
    pub emissive: Vec<f64>,              //Ke
    pub shininess: f64,                  //Ns
    pub transparency: f64,               //d
}

impl From<Vec3> for Vec3d {
    fn from(v: Vec3) -> Vec3d {
        Vec3d {
            x: v.x,
            y: v.y,
            z: v.z,
        }
    }
}
