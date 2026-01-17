use crate::data_plane::{scene::render_scene::Scene, scene_io::file_manager::FileManager};
use glam::Vec3;
use scene_objects::{
    camera::Camera,
    light_source::{LightSource, LightType},
    material::Material,
    sphere::Sphere,
    geometric_object::SceneObject,
};
use std::fs;
use std::path::PathBuf;

fn create_test_scene(name: &str) -> Scene {
    let mut scene = Scene::new();
    scene.set_name(name.to_string());

    // Add a sphere
    scene.add_sphere(Sphere::new(
        Vec3::new(1.0, 2.0, 3.0),
        1.5,
        Material::default(),
        [1.0, 0.0, 0.0],
    ));

    // Add a light
    scene.add_lightsource(LightSource::new(
        Vec3::new(10.0, 10.0, 10.0),
        100.0,
        [1.0, 1.0, 1.0],
        "TestLight".to_string(),
        Vec3::ZERO,
        LightType::Point,
    ));

    // Setup camera
    let mut cam = Camera::default();
    cam.set_position(Vec3::new(0.0, 0.0, -10.0));
    scene.set_camera(cam);

    scene
}

use std::sync::atomic::{AtomicUsize, Ordering};

static COUNTER: AtomicUsize = AtomicUsize::new(0);

fn setup_temp_dir() -> PathBuf {
    let count = COUNTER.fetch_add(1, Ordering::SeqCst);
    let dir = std::env::temp_dir().join(format!(
        "renderbaby_test_{}_{}",
        chrono::Utc::now().timestamp_millis(),
        count
    ));
    fs::create_dir_all(&dir).unwrap();
    dir
}

#[test]
fn test_json_export_import_integrity() {
    let temp_dir = setup_temp_dir();
    let file_path = temp_dir.join("test_scene.json");

    let original_scene = create_test_scene("JsonTest");

    // Export
    original_scene
        .export_scene(file_path.clone(), false)
        .expect("Failed to export JSON scene");

    // Import
    let imported_scene =
        Scene::load_scene_from_path(file_path, false).expect("Failed to import JSON scene");

    // Assertions
    assert_eq!(original_scene.get_name(), imported_scene.get_name());
    // Spheres are not serialized by design in scene_exporter currently, so we don't check them.
}

#[test]
fn test_rscn_export_import_with_mesh() {
    let temp_dir = setup_temp_dir();
    let export_path = temp_dir.join("test_bundle.rscn");

    let mut scene = create_test_scene("RscnTest");

    // We need a real mesh file to test.
    // fixtures/scenes/obj/cube_bare.obj exists in the project root.
    let project_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let fixture_obj = project_root.join("fixtures/scenes/obj/cube_bare.obj");

    assert!(
        fixture_obj.exists(),
        "Fixture obj not found at {:?}",
        fixture_obj
    );

    // Load the mesh into the scene first
    scene
        .load_object_from_file(fixture_obj)
        .expect("Failed to load fixture mesh");

    assert_eq!(scene.get_meshes().len(), 1);
    let original_mesh_name = scene.get_meshes()[0].get_name().clone();

    // Export to .rscn
    scene
        .export_scene(export_path.clone(), false)
        .expect("Failed to export RSCN");
    assert!(export_path.exists());

    // Import back
    let imported_scene =
        Scene::load_scene_from_path(export_path, false).expect("Failed to import RSCN");

    assert_eq!(imported_scene.get_name(), "RscnTest");
    assert_eq!(imported_scene.get_meshes().len(), 1);
    assert_eq!(
        imported_scene.get_meshes()[0].get_name(),
        original_mesh_name
    );

    // Check if the path of the imported mesh is valid and absolute
    let imported_path = imported_scene.get_meshes()[0].get_path().clone().unwrap();
    assert!(imported_path.is_absolute());
    assert!(imported_path.exists());

    // Ensure it is NOT the original path
    assert_ne!(
        imported_path,
        project_root.join("fixtures/scenes/obj/cube_bare.obj")
    );

    // Clean up
    let _ = fs::remove_dir_all(temp_dir);
}

#[test]
fn test_import_nonexistent_file() {
    let temp_dir = setup_temp_dir();
    let path = temp_dir.join("fake.json");
    let result = Scene::load_scene_from_path(path, false);
    assert!(result.is_err());
}

#[test]
fn test_invalid_rscn_structure() {
    let temp_dir = setup_temp_dir();
    let zip_path = temp_dir.join("bad.rscn");

    // Create a zip file without scene.json
    let file = fs::File::create(&zip_path).unwrap();
    let mut zip = zip::ZipWriter::new(file);
    let options = zip::write::FileOptions::<()>::default();
    zip.start_file("dummy.txt", options).unwrap();
    use std::io::Write;
    zip.write_all(b"hello").unwrap();
    zip.finish().unwrap();

    let result = Scene::load_scene_from_path(zip_path, false);
    // This should fail because scene.json is missing
    assert!(result.is_err());
}

#[test]
fn test_idempotency() {
    // Idempotency in this context: Importing the same file twice produces equivalent scenes
    let temp_dir = setup_temp_dir();
    let export_path = temp_dir.join("idem.rscn");

    let mut scene = Scene::new();
    scene.set_name("Idem".to_string());
    // Add a simple light
    scene.add_lightsource(LightSource::new(
        Vec3::ZERO,
        1.0,
        [1.0, 1.0, 1.0],
        "L".into(),
        Vec3::ZERO,
        LightType::Ambient,
    ));

    scene.export_scene(export_path.clone(), false).unwrap();

    let s1 = Scene::load_scene_from_path(export_path.clone(), false).unwrap();
    let s2 = Scene::load_scene_from_path(export_path.clone(), false).unwrap();

    assert_eq!(s1.get_name(), s2.get_name());
    assert_eq!(s1.get_light_sources().len(), s2.get_light_sources().len());

    // Paths will likely differ because they are extracted to unique temp folders each time
    // But the content should be valid for both
}

#[test]
fn test_scene_save_method() {
    let temp_dir = setup_temp_dir();
    let save_path = temp_dir.join("saved_via_method.rscn");

    let mut scene = create_test_scene("SaveMethodTest");

    // Initially no output path, save should fail
    assert!(scene.save(false).is_err());

    // Set path
    scene.set_output_path(Some(save_path.clone())).unwrap();
    assert_eq!(scene.get_output_path(), Some(save_path.clone()));

    // Save should work now
    scene.save(false).expect("Scene::save() failed");

    assert!(save_path.exists());

    // Verify we can load it back
    let loaded = Scene::load_scene_from_path(save_path, false).expect("Failed to load saved scene");
    assert_eq!(loaded.get_name(), "SaveMethodTest");
}

#[test]
fn test_rscn_import_disables_color_hash() {
    let temp_dir = setup_temp_dir();
    let export_path = temp_dir.join("color_hash_test.rscn");

    let scene = create_test_scene("ColorHashTest");
    // Default should be true
    assert!(scene.get_color_hash_enabled());

    scene.export_scene(export_path.clone(), false).unwrap();

    // Load it back
    let loaded = Scene::load_scene_from_path(export_path, false).unwrap();
    // Should be false for rscn
    assert!(!loaded.get_color_hash_enabled());

    let _ = fs::remove_dir_all(temp_dir);
}

#[test]
fn test_rscn_export_with_mtl() {
    let temp_dir = setup_temp_dir();
    let export_path = temp_dir.join("cornell_export.rscn");

    let mut scene = create_test_scene("MtlTest");

    // Load cornell box which has an MTL
    let project_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let fixture_obj = project_root.join("fixtures/cornell_box/cornell-box.obj");

    assert!(
        fixture_obj.exists(),
        "Fixture obj not found at {:?}",
        fixture_obj
    );

    scene
        .load_object_from_file(fixture_obj)
        .expect("Failed to load cornell box");

    // Export
    scene
        .export_scene(export_path.clone(), false)
        .expect("Failed to export RSCN");

    // Verify content by unzipping manually (using FileManager)
    // FileManager::unzip_scene extracts to a temp dir
    let unzipped_path = FileManager::unzip_scene(&export_path).expect("Failed to unzip");

    // Expected structure:
    // unzipped_path/scene/obj/cornell-box.obj
    // unzipped_path/scene/obj/cornell-box.mtl
    let obj_path = unzipped_path.join("scene/obj/cornell-box.obj");
    let mtl_path = unzipped_path.join("scene/obj/cornell-box.mtl");

    assert!(obj_path.exists(), "OBJ file missing in archive");
    assert!(mtl_path.exists(), "MTL file missing in archive");

    // Clean up
    let _ = fs::remove_dir_all(temp_dir);
    let _ = fs::remove_dir_all(unzipped_path);
}
