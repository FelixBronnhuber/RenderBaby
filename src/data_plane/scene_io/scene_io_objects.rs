use glam::Vec3;
use serde::{Deserialize, Serialize};
use scene_objects::material::{Material, MaterialPresets};
use crate::data_plane::scene_proxy::color::Color;

#[derive(Serialize, Deserialize, Debug)]
pub struct SceneFile {
    pub scene_name: String,
    pub objects: Vec<ParsingObject>,
    pub lights: Vec<FileLightSource>,
    pub camera: FileCamera,
    pub background_color: FileColor,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub misc: Option<SceneFileMisc>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SceneFileMisc {
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

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct FileColor {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a: Option<f32>, // ungenutzt
}

impl From<FileColor> for Color {
    fn from(value: FileColor) -> Self {
        Color {
            r: value.r,
            g: value.g,
            b: value.b,
        }
    }
}

impl From<FileColor> for [f32; 3] {
    fn from(c: FileColor) -> [f32; 3] {
        [c.r, c.g, c.b]
    }
}

impl From<&[f32; 3]> for FileColor {
    fn from(value: &[f32; 3]) -> Self {
        FileColor {
            r: value[0],
            g: value[1],
            b: value[2],
            a: None,
        }
    }
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
    pub center: Vec3d,
    pub radius: f32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub material: Option<FileMaterialRef>,
    pub color: FileColor,
    pub name: String,
    pub scale: Vec3d,
    pub translation: Vec3d,
    pub rotation: Vec3d,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum FileMaterialRef {
    Preset { preset: String },
    Path { path: String, name: String },
}

impl TryFrom<&Material> for FileMaterialRef {
    type Error = anyhow::Error;
    fn try_from(mat: &Material) -> anyhow::Result<Self> {
        if let Some(ref_path) = &mat.ref_path {
            Ok(FileMaterialRef::Path {
                path: ref_path.clone(),
                name: mat.name.clone(),
            })
        } else {
            match MaterialPresets::try_from(mat) {
                Ok(preset) => Ok(FileMaterialRef::Preset {
                    preset: preset.into(),
                }),
                Err(e) => Err(e),
            }
        }
    }
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
