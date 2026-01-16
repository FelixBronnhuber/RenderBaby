use std::fs;
use std::path::PathBuf;
use anyhow::Context;
use glam::Vec3;
use scene_objects::{
    camera,
    camera::Camera,
    light_source::{LightSource, LightType},
    sphere::Sphere,
};
use serde_json::json;
use scene_objects::material::Material;
use crate::data_plane::scene::{render_scene::Scene};
use crate::data_plane::scene_io::scene_io_objects::*;
#[allow(dead_code)]
pub struct SceneParseResult {
    pub scene: Scene,
    pub obj_paths: Vec<String>,
    pub rotation: Vec<Vec3>,
    pub translation: Vec<Vec3>,
    pub scale: Vec<Vec3>,
}
fn transform_to_scene(file: SceneFile) -> anyhow::Result<SceneParseResult> {
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
    scene
        .get_camera_mut()
        .set_pane_width(file.camera.pane_width);
    scene
        .get_camera_mut()
        .set_pane_distance(file.camera.pane_distance);
    //Background
    scene.set_background_color([
        file.background_color.r,
        file.background_color.g,
        file.background_color.b,
    ]);
    let obj_paths = file
        .objects
        .iter()
        .map(|o| o.path.clone())
        .collect::<Vec<String>>();
    let rotation = file
        .objects
        .iter()
        .map(|o| Vec3::new(o.rotation.x, o.rotation.y, o.rotation.z))
        .collect();
    let translation = file
        .objects
        .iter()
        .map(|o| Vec3::new(o.translation.x, o.translation.y, o.translation.z))
        .collect();
    let scale = file
        .objects
        .iter()
        .map(|o| Vec3::new(o.scale.x, o.scale.y, o.scale.z))
        .collect();

    //Spheres
    if let Some(file_spheres) = file.spheres {
        file_spheres.iter().for_each(|file_sphere| {
            scene.add_sphere(Sphere::new(
                file_sphere.center,
                file_sphere.radius,
                Material::new(
                    file_sphere.material.name.clone(),
                    file_sphere.material.ambient_reflectivity.clone(),
                    file_sphere.material.diffuse_reflectivity.clone(),
                    file_sphere.material.specular_reflectivity.clone(),
                    file_sphere.material.emissive.clone(),
                    file_sphere.material.shininess,
                    file_sphere.material.transparency,
                    None,
                ),
                [
                    file_sphere.color.r,
                    file_sphere.color.g,
                    file_sphere.color.b,
                ],
            ))
        })
    };

    //set ray_samples
    if let Some(samples) = file.ray_samples {
        scene.get_camera_mut().set_ray_samples(samples);
    }

    //set hash_coloring
    if let Some(coloring) = file.hash_color {
        scene.set_color_hash_enabled(coloring);
    }

    Ok(SceneParseResult {
        scene,
        obj_paths,
        rotation,
        translation,
        scale,
    })
}

pub fn parse_scene(
    scene_path: PathBuf,
    json_string: Option<String>,
) -> anyhow::Result<SceneParseResult> {
    let mut _json_content: String = String::new();
    match json_string {
        Some(json_string) => {
            _json_content = json_string;
        }
        None => {
            if !scene_path.is_file() {
                return Err(anyhow::Error::msg(format!(
                    "File {} does not exist!",
                    scene_path.display()
                )));
            }
            _json_content = fs::read_to_string(scene_path).context("file could not be read")?;
        }
    }

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
        "additionalProperties": true,

        "properties": {
            "scene_name": {
                "type": "string",
                "minLength": 0
            },

            "objects": {
                "type": "array",
                "minItems": 0,
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
                "minItems": 0,
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
            },

            "sphere": {
                "type": "array",
                "required": [
                    "center",
                    "radius",
                    "material",
                    "color",
                    "name",
                    "scale",
                    "translation",
                    "rotation",
                ],
                "properties": {
                    "center": {"$ref": "#/$defs/vec3"},
                    "radius": {"type": "number"},
                    "material": {"$ref": "#/$defs/sphereMaterial"},
                    "color": {"$ref": "#/$defs/color"},
                    "name": {"type": "string"},
                    "scale": {"$ref": "#/$defs/vec3"},
                    "translation": {"$ref": "#/$defs/vec3"},
                    "rotation": {"$ref": "#/$defs/vec3"},
                },
                "additionalProperties": false,
            },

            "sphereMaterial": {
                "type": "object",
                "required:": [
                    "name",
                    "ambient_reflectivity",
                    "diffuse_reflectivity",
                    "specular_reflectivity",
                    "emissive",
                    "shininess",
                    "transparency",
                ],
                "properties": {
                    "name": {"type": "string"},
                    "ambient_reflectivity": {"$ref": "#/$defs/vec3"},
                    "diffuse_reflectivity": {"$ref": "#/$defs/vec3"},
                    "specular_reflectivity": {"$ref": "#/$defs/vec3"},
                    "emissive": {"$ref": "#/$defs/vec3"},
                    "shininess": {"type": "number"},
                    "transparency": {"type": "number"},
                },
                "additionalProperties": false,
            },

            "ray_samples": {
                "type": "number",
            },

            "hash_color": {
                "type": "boolean",
            }
        }
    });
    let json_value = serde_json::from_str(_json_content.clone().as_str())?;
    if jsonschema::is_valid(&schema, &json_value) {
        let read = serde_json::from_str::<SceneFile>(&_json_content)?;
        let res = transform_to_scene(read)?;
        Result::Ok(res)
    } else {
        Err(anyhow::Error::msg(
            "JSON does not comply with Schema".to_string(),
        ))
    }
}
