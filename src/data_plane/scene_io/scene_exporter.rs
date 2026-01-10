use std::fs::File;
use std::path::PathBuf;
use scene_objects::geometric_object::SceneObject;
use scene_objects::light_source::*;
use crate::data_plane::scene::render_scene::Scene;
use crate::data_plane::scene_io::scene_io_objects::*;

pub fn serialize_scene(path: PathBuf, sc: &Scene) -> anyhow::Result<()> {
    let mut objects: Vec<ParsingObject> = Vec::with_capacity(2);
    let mut lightarr: Vec<FileLightSource> = Vec::new();
    let scene_name = sc.get_name().clone();

    if let Some(directory) = path.parent() {
        //objects
        sc.get_meshes().iter().for_each(|object| {
            println!("{:?}", object.get_path());
            let written_path;

            //if path is absolute (obj_import) or path is relative (scene_import)
            if let Ok(relative_path) = object
                .get_path()
                .unwrap_or_default()
                .strip_prefix(directory)
            {
                written_path = relative_path.to_string_lossy().to_string();
            } else {
                written_path = object
                    .get_path()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_string();
            }
            objects.push(ParsingObject {
                name: object.get_name(),
                path: written_path,
                scale: object.get_scale().into(),
                translation: object.get_translation().into(),
                rotation: object.get_rotation().into(),
            });
        });

        //lights
        sc.get_light_sources().iter().for_each(|light_source| {
            let colors = light_source.get_color();
            lightarr.push(FileLightSource {
                name: light_source.get_name().clone(),
                r#type: match light_source.get_light_type() {
                    LightType::Ambient => "ambient".to_owned(),
                    LightType::Point => "point".to_owned(),
                    LightType::Directional => "directional".to_owned(),
                },
                position: light_source.get_position().into(),
                luminosity: light_source.get_luminositoy(),
                color: FileColor {
                    r: colors[0],
                    g: colors[1],
                    b: colors[2],
                    a: None,
                },
                rotation: Some(light_source.get_rotation().into()),
            })
        });

        //camera
        let camera = sc.get_camera();
        let file_camera = FileCamera {
            position: camera.get_position().into(),
            look_at: camera.get_look_at().into(),
            up: camera.get_up().into(),
            pane_distance: camera.get_pane_distance(),
            pane_width: camera.get_pane_width(),
            resolution: Resolution {
                x: camera.get_resolution().width,
                y: camera.get_resolution().height,
            },
        };

        //background
        let bg = sc.get_background_color();

        let final_scene = SceneFile {
            scene_name,
            objects,
            lights: lightarr,
            camera: file_camera,
            background_color: FileColor {
                r: bg[0],
                g: bg[1],
                b: bg[2],
                a: None,
            },
        };
        println!("final_scene_export: {:?}", final_scene);
        let output = File::create(path);
        match output {
            Ok(output) => {
                serde_json::to_writer_pretty(output, &final_scene)
                    .expect("Could not write scene into file");
                Ok(())
            }
            Err(error) => Err(error.into()),
        }
    } else {
        Err(anyhow::Error::msg("Path has no parent"))
    }
}
