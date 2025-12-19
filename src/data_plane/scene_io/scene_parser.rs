use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::ops::Deref;
use std::path::PathBuf;

use glam::Vec3;
use scene_objects::{camera, camera::Camera, geometric_object::SceneObject, light_source::{LightSource, LightType}, tri_geometry::TriGeometry};
use serde::{Deserialize, Serialize};

use crate::data_plane::scene::{render_scene::Scene};
#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug)]
struct SceneFile {
    scene_name: String,
    objects: Vec<ParsingObject>,
    lights: Vec<FileLightSource>,
    camera: FileCamera,
    background_color: FileColor,
}

#[derive(Serialize, Deserialize, Debug)]
struct ParsingObject {
    name: String,
    pub path: String,
    scale: Vec3d,
    translation: Vec3d,
    rotation: Vec3d, //x = roll, y = pitch, z = yaw
}

#[derive(Serialize, Deserialize, Debug)]
struct FileLightSource {
    name: String,
    r#type: String,
    position: Vec3d,
    luminosity: f32,
    color: FileColor,
    rotation: Option<Vec3d>,
}

#[derive(Serialize, Deserialize, Debug)]
struct FileCamera {
    position: Vec3d,
    look_at: Vec3d,
    up: Vec3d, //roll
    pane_distance: f32,
    pane_width: f32,
    resolution: Resolution,
}

#[derive(Serialize, Deserialize, Debug)]
struct FileColor {
    r: f32,
    g: f32,
    b: f32,
    a: Option<f32>, //ungenutzt
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Vec3d {
    x: f32,
    y: f32,
    z: f32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Resolution {
    x: u32,
    y: u32
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
#[allow(dead_code)]
fn transform_to_scene(file: SceneFile) -> (Scene, Vec<String>) {
    let mut scene = Scene::new();

    //name
    scene.set_name(file.scene_name);

    //lights
    file.lights.iter().for_each(|light| {
        scene.add_lightsource(LightSource::new(
            Vec3::new(light.position.x, light.position.y, light.position.z),
            light.luminosity,
            [light.color.r, light.color.g, light.color.b],
            light.name.clone(),
            {
                if light.rotation.is_some() {
                    let rotation = light.rotation.clone().unwrap();
                    Vec3::new(rotation.x, rotation.y, rotation.z)
                }else {
                    Vec3::new(0.0, 0.0, 0.0)
                }
            },
            match light.r#type.as_str() {
                "ambient" => LightType::Ambient,
                "point" => LightType::Point,
                "directional" => LightType::Directional,
                _ => LightType::Ambient,
            },
        ))
    });

    //camera
    let (fx, fy, fz): (f32, f32, f32) = (
        file.camera.look_at.x / 3.0 - file.camera.position.x / 3.0,
        file.camera.look_at.y / 3.0 - file.camera.position.y / 3.0,
        file.camera.look_at.z / 3.0 - file.camera.position.z / 3.0,
    );
    let yaw = fx.atan2(fz);
    let pitch = fy.atan2((fx * fx + fz * fz).sqrt());
    scene.set_camera(Camera::new(
        Vec3::new(
            file.camera.position.x,
            file.camera.position.y,
            file.camera.position.z,
        ),
        Vec3::new(pitch, yaw, 0.0),
    ));
    scene.get_camera_mut().set_resolution(camera::Resolution::new(file.camera.resolution.x,file.camera.resolution.y));
    let fov_rad = 2.0 * (file.camera.pane_width / (2.0 * file.camera.pane_distance)).atan();
    let fov_deg = fov_rad.to_degrees();
    scene.get_camera_mut().set_fov(fov_deg);

    //Background
    scene.set_background_color([
        file.background_color.r,
        file.background_color.g,
        file.background_color.b,
    ]);
    let paths = file.objects.iter().map(|o| o.path.clone()).collect::<Vec<String>>();
    (scene, paths)
}
pub fn parse_scene(scene_path: PathBuf) -> anyhow::Result<(Scene, Vec<String>)> {
    // TODO: please add proper error handling!!!
    if !scene_path.is_file() {
        return Err(anyhow::Error::msg(format!(
            "File {} does not exist!",
            scene_path.display()
        )));
    }
    let _json_content = fs::read_to_string(scene_path)?;
    let read = serde_json::from_str::<SceneFile>(&_json_content)?;
    let res = transform_to_scene(read);
    Result::Ok(res)
}
