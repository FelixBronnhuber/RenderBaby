use std::fs::{self, File};
use std::io::BufRead;
use std::path::{Path, PathBuf};
use scene_objects::geometric_object::SceneObject;
use scene_objects::light_source::*;
use crate::data_plane::scene::render_scene::Scene;
use crate::data_plane::scene_io::scene_io_objects::*;
use crate::data_plane::scene_io::file_manager::FileManager;
use log::{info, debug, warn};

use std::sync::atomic::{AtomicUsize, Ordering};

static EXPORT_COUNTER: AtomicUsize = AtomicUsize::new(0);

pub fn serialize_scene(path: PathBuf, sc: &Scene, export_misc: bool) -> anyhow::Result<()> {
    info!(
        "SceneExporter: Exporting scene '{}' to {:?}",
        sc.get_name(),
        path
    );
    let is_rscn = path.extension().map(|s| s == "rscn").unwrap_or(false);

    let staging_dir = if is_rscn {
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos();
        let count = EXPORT_COUNTER.fetch_add(1, Ordering::SeqCst);
        let d = std::env::temp_dir()
            .join("renderbaby")
            .join(format!("export_{}_{}", nanos, count));
        debug!("SceneExporter: Created staging directory {:?}", d);
        fs::create_dir_all(d.join("scene/obj"))?;
        Some(d)
    } else {
        None
    };

    let base_dir = if let Some(ref d) = staging_dir {
        d.join("scene")
    } else {
        path.parent()
            .ok_or_else(|| anyhow::Error::msg("Path has no parent"))?
            .to_path_buf()
    };

    let mut objects: Vec<ParsingObject> = Vec::with_capacity(sc.get_meshes().len());
    let mut lightarr: Vec<FileLightSource> = Vec::new();
    let scene_name = sc.get_name().clone();

    //objects
    for object in sc.get_meshes() {
        let written_path;

        if is_rscn {
            // Copy file to staging/scene/obj/
            let src = object.get_path().unwrap_or_default();

            let filename = src
                .file_name()
                .ok_or_else(|| anyhow::Error::msg("Invalid source file name"))?;

            let dest_rel = PathBuf::from("obj").join(filename);
            let dest_abs = base_dir.join(&dest_rel);

            if let Some(p) = dest_abs.parent()
                && !p.exists()
            {
                fs::create_dir_all(p)?;
            }

            // It's possible the file doesn't exist if it was procedural or moved.
            // We attempt copy.
            if src.exists() {
                debug!(
                    "SceneExporter: Copying asset from {:?} to {:?}",
                    src, dest_abs
                );
                fs::copy(&src, &dest_abs)?;

                // Recursively copy dependencies (MTL, textures)
                if let Err(e) = copy_obj_dependencies(&src, &dest_abs) {
                    warn!(
                        "SceneExporter: Error copying dependencies for {:?}: {}",
                        src, e
                    );
                }
            } else {
                warn!(
                    "SceneExporter: Asset file not found at {:?}, skipping copy.",
                    src
                );
            }

            // Path in JSON should be relative to scene.json (which is in base_dir)
            let p_str = dest_rel.to_string_lossy().to_string();
            #[cfg(windows)]
            let p_str = p_str.replace('\\', "/");

            written_path = p_str;
        } else {
            // if path is absolute (obj_import) or else path is relative (scene_import)
            if let Ok(relative_path) = object
                .get_path()
                .unwrap_or_default()
                .strip_prefix(&base_dir)
            {
                written_path = relative_path.to_string_lossy().to_string();
            } else if let Ok(relative_path) = object
                .get_path()
                .unwrap_or_default()
                .strip_prefix(base_dir.parent().unwrap())
            {
                let mut path = PathBuf::from("../");
                path.push(relative_path);
                written_path = path.to_string_lossy().to_string();
            } else {
                written_path = object
                    .get_path()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_string();
            }
        }

        objects.push(ParsingObject {
            name: object.get_name(),
            path: written_path,
            scale: object.get_scale().into(),
            translation: object.get_translation().into(),
            rotation: object.get_rotation().into(),
        });
    }

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

    //spheres
    let spheres = sc.get_spheres();
    let mut file_spheres = Vec::new();
    if !spheres.is_empty() {
        spheres.iter().for_each(|sphere| {
            let material = sphere.get_material();
            file_spheres.push(FileSphere {
                center: sphere.get_center().into(),
                radius: sphere.get_radius(),
                material: FileMaterial {
                    name: material.name.clone(),
                    ambient_reflectivity: material.ambient_reflectivity.clone(), //Ka
                    diffuse_reflectivity: material.diffuse_reflectivity.clone(), //Kd
                    specular_reflectivity: material.specular_reflectivity.clone(), //Ks
                    emissive: material.emissive.clone(),                         //Ke
                    shininess: material.shininess,                               //Ns
                    transparency: material.transparency,                         //d
                },
                color: FileColor {
                    r: sphere.get_color()[0],
                    g: sphere.get_color()[1],
                    b: sphere.get_color()[2],
                    a: None,
                },
                name: "Sphere".to_owned(),
                scale: sphere.get_scale().into(),
                translation: sphere.get_translation().into(),
                rotation: sphere.get_rotation().into(),
            })
        });
    }

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
        spheres: if export_misc {
            Some(file_spheres)
        } else {
            None
        },
        ray_samples: if export_misc {
            Some(sc.get_camera().get_ray_samples())
        } else {
            None
        },
        hash_color: if export_misc {
            Some(sc.get_color_hash_enabled())
        } else {
            None
        },
    };

    let json_output_path = if is_rscn {
        base_dir.join("scene.json")
    } else {
        path.clone()
    };

    info!(
        "SceneExporter: Writing scene JSON to {:?}",
        json_output_path
    );
    let output = File::create(&json_output_path)?;
    serde_json::to_writer_pretty(output, &final_scene).expect("Could not write scene into file");

    if is_rscn {
        // Zip staging_dir (which contains 'scene' folder) to 'path'
        info!("SceneExporter: Compressing staging directory to .rscn...");
        FileManager::zip_scene(staging_dir.as_ref().unwrap(), &path)?;

        // cleanup
        debug!(
            "SceneExporter: Cleaning up staging directory {:?}",
            staging_dir
        );
        let _ = fs::remove_dir_all(staging_dir.unwrap());
    }

    info!("SceneExporter: Export completed successfully.");
    Ok(())
}

fn copy_obj_dependencies(src_obj: &Path, dest_obj: &Path) -> anyhow::Result<()> {
    // Basic parser to find mtllib in OBJ
    let file = File::open(src_obj)?;
    let reader = std::io::BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        let trimmed = line.trim();
        if trimmed.starts_with("mtllib") {
            let parts: Vec<&str> = trimmed.split_whitespace().collect();
            if parts.len() > 1 {
                let mtl_filename = parts[1];
                resolve_and_copy(src_obj, dest_obj, mtl_filename, true)?;
            }
        }
    }
    Ok(())
}

fn resolve_and_copy(
    ref_src: &Path,
    ref_dest: &Path,
    relative_path: &str,
    is_mtl: bool,
) -> anyhow::Result<()> {
    let src_parent = ref_src
        .parent()
        .ok_or_else(|| anyhow::Error::msg("Invalid source parent"))?;
    let dest_parent = ref_dest
        .parent()
        .ok_or_else(|| anyhow::Error::msg("Invalid destination parent"))?;

    let src_path = src_parent.join(relative_path);
    let dest_path = dest_parent.join(relative_path);

    if !src_path.exists() {
        warn!(
            "SceneExporter: Referenced file not found: {:?} (referenced from {:?})",
            src_path, ref_src
        );
        return Ok(());
    }

    if let Some(p) = dest_path.parent()
        && !p.exists()
    {
        fs::create_dir_all(p)?;
    }

    if !dest_path.exists() {
        debug!(
            "SceneExporter: Copying dependency from {:?} to {:?}",
            src_path, dest_path
        );
        fs::copy(&src_path, &dest_path)?;
    }

    // Look for textures inside mtl
    if is_mtl {
        let file = File::open(&src_path)?;
        let reader = std::io::BufReader::new(file);
        for line in reader.lines() {
            let line = line?;
            let trimmed = line.trim();
            if trimmed.starts_with("map_") || trimmed.starts_with("bump") {
                let parts: Vec<&str> = trimmed.split_whitespace().collect();
                if parts.len() > 1
                    && let Some(tex_filename) = parts.last()
                {
                    resolve_and_copy(&src_path, &dest_path, tex_filename, false)?;
                }
            }
        }
    }

    Ok(())
}
