use std::fs;
use std::path::{Path, PathBuf};
use anyhow::Context;
use glam::Vec3;
use scene_objects::{
    camera,
    camera::Camera,
    light_source::{LightSource, LightType},
};
use crate::data_plane::scene::{render_scene::Scene};
use crate::data_plane::scene_io::scene_io_objects::*;
use crate::data_plane::scene_io::file_manager::FileManager;
use log::{info, debug, error};

pub struct LoadedSceneData {
    pub scene: Scene,
    pub paths: Vec<String>,
    pub rotations: Vec<Vec3>,
    pub translations: Vec<Vec3>,
    pub scales: Vec<Vec3>,
}

#[allow(dead_code)]
#[allow(clippy::type_complexity)]
fn transform_to_scene(file: SceneFile) -> anyhow::Result<LoadedSceneData> {
    debug!(
        "SceneImporter: Transforming parsed SceneFile to Scene object. Name: {}",
        file.scene_name
    );
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
    let paths = file
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
    Ok(LoadedSceneData {
        scene,
        paths,
        rotations: rotation,
        translations: translation,
        scales: scale,
    })
}

#[allow(clippy::type_complexity)]
pub fn parse_scene(
    scene_path: PathBuf,
    json_string: Option<String>,
) -> anyhow::Result<LoadedSceneData> {
    if json_string.is_some() {
        info!("SceneImporter: Parsing scene from JSON string.");
    } else {
        info!("SceneImporter: Parsing scene from file: {:?}", scene_path);
    }

    let is_rscn = scene_path.extension().map(|s| s == "rscn").unwrap_or(false);

    let (json_content, base_path) = if is_rscn {
        info!("SceneImporter: Detected .rscn file, invoking FileManager...");
        let temp_root = FileManager::unzip_scene(&scene_path)?;
        let json_path = FileManager::find_scene_json(&temp_root)?;
        info!(
            "SceneImporter: Found internal scene.json at {:?}",
            json_path
        );
        let content = fs::read_to_string(&json_path)?;
        debug!(
            "SceneImporter: Read JSON content from {:?}:\n{}",
            json_path, content
        );
        (content, json_path.parent().unwrap().to_path_buf())
    } else if let Some(s) = json_string {
        (
            s,
            scene_path.parent().unwrap_or(Path::new(".")).to_path_buf(),
        )
    } else {
        if !scene_path.is_file() {
            error!("SceneImporter: File not found: {:?}", scene_path);
            return Err(anyhow::Error::msg(format!(
                "File {} does not exist!",
                scene_path.display()
            )));
        }
        let content = fs::read_to_string(&scene_path).context("file could not be read")?;
        (content, scene_path.parent().unwrap().to_path_buf())
    };

    let schema_json = include_str!("scene_schema.json");
    let schema: serde_json::Value = serde_json::from_str(schema_json)?;

    debug!("SceneImporter: Validating JSON schema...");
    let json_value = serde_json::from_str(&json_content)?;
    if jsonschema::is_valid(&schema, &json_value) {
        debug!("SceneImporter: JSON schema validation successful.");
        let read = serde_json::from_str::<SceneFile>(&json_content)?;
        let mut loaded_data = transform_to_scene(read)?;

        let abs_paths = loaded_data
            .paths
            .into_iter()
            .map(|p| {
                let abs = base_path.join(&p).to_string_lossy().to_string();
                debug!("SceneImporter: Resolved asset path: {} -> {}", p, abs);
                abs
            })
            .collect();
        loaded_data.paths = abs_paths;

        info!("SceneImporter: Scene parsing successful.");
        Ok(loaded_data)
    } else {
        error!("SceneImporter: JSON schema validation failed.");
        Err(anyhow::Error::msg(
            "JSON does not comply with Schema".to_string(),
        ))
    }
}
