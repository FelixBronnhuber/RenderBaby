use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::path::PathBuf;

use glam::Vec3;
use scene_objects::{
    camera::Camera,
    geometric_object::SceneObject,
    light_source::{LightSource, LightType},
    tri_geometry::TriGeometry,
};
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
    path: String,
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
    resolution: HashMap<String, u32>,
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
fn transform_to_scene(file: SceneFile) -> Scene {
    let mut scene = Scene::new();
    scene.set_name(file.scene_name);
    file.lights.iter().for_each(|light| {
        scene.add_lightsource(LightSource::new(
            Vec3::new(light.position.x, light.position.y, light.position.z),
            light.luminosity,
            [light.color.r, light.color.g, light.color.b],
            light.name.clone(),
            {
                let rotation = light.rotation.clone().unwrap();
                Vec3::new(rotation.x, rotation.y, rotation.z)
            },
            match light.r#type.as_str() {
                "ambient" => LightType::Ambient,
                "point" => LightType::Point,
                "directional" => LightType::Directional,
                _ => LightType::Ambient,
            },
        ))
    });
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

    scene.add_tri_geometry(TriGeometry::new(Vec::new()));
    scene.set_background_color([
        file.background_color.r,
        file.background_color.g,
        file.background_color.b,
    ]);
    scene
}
#[allow(dead_code)]
pub fn serialize_scene(sc: &mut Scene) {
    //if let Some(p) = obj.as_any().downcast_ref::<Player>() ;
    //let scene_file = Scene_File{scene_name : sc.get_name().to_string(),objects : sc.get_objects().for_each().,camera: sc.get_camera(), lights: sc.get_light_sources(), background_color: sc.get_background_color()};
    let mut objectarr: Vec<ParsingObject> = Vec::new();
    sc.get_tri_geometries().iter().for_each(|object| {
        objectarr.push(ParsingObject {
            name: object.get_name().clone(),
            path: {
                match object.get_path() {
                    Some(p) => p.to_owned(),
                    None => "".to_owned(),
                }
            },
            scale: Vec3d {
                x: object.get_scale().x,
                y: object.get_scale().y,
                z: object.get_scale().z,
            },
            rotation: Vec3d {
                x: object.get_rotation().x,
                y: object.get_rotation().y,
                z: object.get_rotation().z,
            },
            translation: Vec3d {
                x: object.get_translation().x,
                y: object.get_translation().y,
                z: object.get_translation().z,
            },
        });
    });
    let mut lightarr: Vec<FileLightSource> = Vec::new();
    sc.get_light_sources().iter().for_each(|light_source| {
        lightarr.push(FileLightSource {
            name: light_source.get_name().to_string(),
            r#type: match light_source.get_light_type() {
                LightType::Ambient => "ambient".to_owned(),
                LightType::Point => "point".to_owned(),
                LightType::Directional => "directional".to_owned(),
            },
            position: Vec3d {
                x: light_source.get_position().x,
                y: light_source.get_position().y,
                z: light_source.get_position().z,
            },
            luminosity: light_source.get_luminositoy(),
            color: FileColor {
                r: light_source.get_color()[0],
                g: light_source.get_color()[1],
                b: light_source.get_color()[2],
                a: None,
            },
            rotation: Some(Vec3d {
                x: light_source.get_rotation().x,
                y: light_source.get_rotation().y,
                z: light_source.get_rotation().z,
            }),
        })
    });

    let mut map = HashMap::with_capacity(2);
    map.insert("x".to_owned(), sc.get_camera().get_resolution().width);
    map.insert("y".to_owned(), sc.get_camera().get_resolution().height);
    let _file = SceneFile {
        scene_name: sc.get_name().to_string(),
        objects: objectarr,
        lights: lightarr,
        camera: FileCamera {
            position: Vec3d {
                x: sc.get_camera().get_position().x,
                y: sc.get_camera().get_position().y,
                z: sc.get_camera().get_position().z,
            },
            look_at: Vec3d {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            up: Vec3d {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            pane_distance: 0.0,
            pane_width: 0.0,
            resolution: map,
        },
        background_color: FileColor {
            r: sc.get_background_color()[0],
            g: sc.get_background_color()[1],
            b: sc.get_background_color()[2],
            a: None,
        },
    };

    let _output = File::create("out.json").expect("Could not create file");
    // serde_json::to_writer_pretty(output, &file).expect("Could not write into file");
    todo!("Fix serde_json");
}

pub fn parse_scene(scene_path: PathBuf) -> anyhow::Result<Scene> {
    // TODO: please add proper error handling!!!
    if !scene_path.is_file() {
        return Err(anyhow::Error::msg(format!(
            "File {} does not exist!",
            scene_path.display()
        )));
    }
    let _json_content = fs::read_to_string(scene_path).unwrap();
    let read = serde_json::from_str::<SceneFile>(&_json_content).unwrap();
    // transform_to_scene(read)
    todo!("Fix serde_json");
}
