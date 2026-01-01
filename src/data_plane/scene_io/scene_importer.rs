use std::fs;
use std::path::PathBuf;
use anyhow::Context;
use glam::Vec3;
use scene_objects::{
    camera,
    camera::Camera,
    light_source::{LightSource, LightType},
};
use serde::{Deserialize, Serialize};
use serde_json::json;
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
    y: u32,
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
fn transform_to_scene(file: SceneFile) -> anyhow::Result<(Scene, Vec<String>)> {
    let mut scene = Scene::new();

    //name
    scene.set_name(file.scene_name);

    //lights
    for light in file.lights {
        scene.add_lightsource(LightSource::new(
            Vec3::new(light.position.x, light.position.y, light.position.z),
            light.luminosity,
            [light.color.r, light.color.g, light.color.b],
            light.name.clone(),
            {
                if light.rotation.is_some() {
                    let rotation = light.rotation.clone().unwrap();
                    Vec3::new(rotation.x, rotation.y, rotation.z)
                } else {
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
    }

    //camera
    scene.set_camera(Camera::new(
        Vec3::new(
            file.camera.position.x,
            file.camera.position.y,
            file.camera.position.z,
        ),
        Vec3::new(
            file.camera.look_at.x,
            file.camera.look_at.y,
            file.camera.look_at.z,
        ),
    ));
    scene
        .get_camera_mut()
        .set_resolution(camera::Resolution::new(
            file.camera.resolution.x,
            file.camera.resolution.y,
        ));
    scene.get_camera_mut().pane_width = file.camera.pane_width;
    scene.get_camera_mut().pane_distance = file.camera.pane_distance;
    //Background
    scene.set_background_color([
        file.background_color.r,
        file.background_color.g,
        file.background_color.b,
    ]);
    let paths = file
        .objects
        .iter()
        .map(|o| o.path.clone())
        .collect::<Vec<String>>();
    Ok((scene, paths))
}
pub fn parse_scene(scene_path: PathBuf) -> anyhow::Result<(Scene, Vec<String>)> {
    if !scene_path.is_file() {
        return Err(anyhow::Error::msg(format!(
            "File {} does not exist!",
            scene_path.display()
        )));
    }

    let _json_content = fs::read_to_string(scene_path).context("file could not be read")?;

    let schema = json!({
        "$schema": "https://json-schema.org/draft/2020-12/schema",
        "type": "object",
        "required": [
        "scene_name",
        "objects",
        "background_color",
        "camera",
        "lights"
        ],
        "additionalProperties": false,

        "properties": {
            "scene_name": {
                "type": "string",
                "minLength": 0
            },

            "objects": {
                "type": "array",
                "minItems": 1,
                "items": { "$ref": "#/$defs/object" }
            },

            "background_color": {
                "$ref": "#/$defs/color"
            },

            "camera": {
                "$ref": "#/$defs/camera"
            },

            "lights": {
                "type": "array",
                "minItems": 1,
                "items": { "$ref": "#/$defs/light" }
            }
        },

        "$defs": {
            "vec3": {
                "type": "object",
                "required": ["x", "y", "z"],
                "additionalProperties": false,
                "properties": {
                    "x": { "type": "number" },
                    "y": { "type": "number" },
                    "z": { "type": "number" }
                }
            },

            "color": {
                "type": "object",
                "required": ["r", "g", "b"],
                "additionalProperties": {"a":{"type":  "number", "minimum": 0.0, "maximum":  1.0}},
                "properties": {
                    "r": { "type": "number", "minimum": 0.0, "maximum": 1.0 },
                    "g": { "type": "number", "minimum": 0.0, "maximum": 1.0 },
                    "b": { "type": "number", "minimum": 0.0, "maximum": 1.0 }
                }
            },

            "object": {
                "type": "object",
                "required": [
                "name",
                "path",
                "scale",
                "rotation",
                "translation"
                ],
                "additionalProperties": false,
                "properties": {
                    "name": { "type": "string" },
                    "path": { "type": "string" },
                    "scale": { "$ref": "#/$defs/vec3" },
                    "rotation": { "$ref": "#/$defs/vec3" },
                    "translation": { "$ref": "#/$defs/vec3" }
                }
            },

            "camera": {
                "type": "object",
                "required": [
                "position",
                "look_at",
                "up",
                "pane_distance",
                "pane_width",
                "resolution"
                ],
                "additionalProperties": false,
                "properties": {
                    "position": { "$ref": "#/$defs/vec3" },
                    "look_at": { "$ref": "#/$defs/vec3" },
                    "up": { "$ref": "#/$defs/vec3" },

                    "pane_distance": {
                        "type": "number",
                        "exclusiveMinimum": 0
                    },

                    "pane_width": {
                        "type": "number",
                        "exclusiveMinimum": 0
                    },

                    "resolution": {
                        "type": "object",
                        "required": ["x", "y"],
                        "additionalProperties": false,
                        "properties": {
                            "x": { "type": "integer", "minimum": 1 },
                            "y": { "type": "integer", "minimum": 1 }
                        }
                    }
                }
            },

            "light": {
                "type": "object",
                "required": [
                "name",
                "type",
                "luminosity",
                "position",
                "color"
                ],
                "additionalProperties": {"rotation": {"$ref": "#/$defs/vec3"}},
                "properties": {
                    "name": { "type": "string" },

                    "type": {
                        "type": "string",
                        "enum": ["ambient", "point", "directional"]
                    },

                    "luminosity": {
                        "type": "number",
                        "exclusiveMinimum": 0
                    },

                    "position": { "$ref": "#/$defs/vec3" },

                    "color": { "$ref": "#/$defs/color" }
                }
            }
        }
    });
    let json_value = serde_json::from_str(_json_content.clone().as_str()).context("value error")?;
    if jsonschema::is_valid(&schema,&json_value) {
        let read = serde_json::from_str::<SceneFile>(&_json_content).context("invalid JSON")?;
    let res = transform_to_scene(read)
            .context("JSON content could not be properly transformed into scene")?;
        Result::Ok(res)
    }else {
        return Err(anyhow::Error::msg(format!("JSON does not comply with Schema")));}
}
