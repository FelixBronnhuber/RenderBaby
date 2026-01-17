#[cfg(test)]
mod tests {
    use glam::Vec3;
    use crate::data_plane::scene::render_scene::Scene;
    use crate::data_plane::scene_io::{scene_exporter, scene_importer};
    use scene_objects::sphere::Sphere;
    use scene_objects::material::Material;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn get_temp_file_path(name: &str) -> std::path::PathBuf {
        let mut dir = std::env::temp_dir();
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .subsec_nanos();
        dir.push(format!("renderbaby_test_{}_{}", nanos, name));
        dir
    }

    #[test]
    fn test_export_import_misc_data() {
        let file_path = get_temp_file_path("test_scene.json");

        let mut scene = Scene::new();
        scene.set_name("Test Scene".to_string());

        // Add a sphere
        let sphere = Sphere::new(
            Vec3::new(1.0, 2.0, 3.0),
            1.5,
            Material::default(),
            [1.0, 0.0, 0.0],
        );
        scene.add_sphere(sphere);

        // Set ray samples and hash color
        scene.get_camera_mut().set_ray_samples(10);
        scene.set_color_hash_enabled(false);

        // Export with export_misc = true
        scene_exporter::serialize_scene(file_path.clone(), &scene, true).expect("Export failed");

        // Import back
        let loaded_data =
            scene_importer::parse_scene(file_path.clone(), None).expect("Import failed");
        let loaded_scene = loaded_data.scene;

        // Verify sphere
        assert_eq!(loaded_scene.get_spheres().len(), 1);
        let loaded_sphere = &loaded_scene.get_spheres()[0];
        assert_eq!(loaded_sphere.get_center(), Vec3::new(1.0, 2.0, 3.0));
        assert_eq!(loaded_sphere.get_radius(), 1.5);

        // Verify ray samples
        assert_eq!(loaded_scene.get_camera().get_ray_samples(), 10);

        // Verify hash color
        assert_eq!(loaded_scene.get_color_hash_enabled(), false);
    }

    #[test]
    fn test_export_misc_data_disabled() {
        let file_path = get_temp_file_path("test_scene_no_misc.json");

        let mut scene = Scene::new();
        scene.set_name("Test Scene No Misc".to_string());

        let sphere = Sphere::new(
            Vec3::new(1.0, 2.0, 3.0),
            1.5,
            Material::default(),
            [1.0, 0.0, 0.0],
        );
        scene.add_sphere(sphere);
        scene.get_camera_mut().set_ray_samples(10);
        scene.set_color_hash_enabled(false);

        // Export with export_misc = false
        scene_exporter::serialize_scene(file_path.clone(), &scene, false).expect("Export failed");

        // Import back
        let loaded_data =
            scene_importer::parse_scene(file_path.clone(), None).expect("Import failed");
        let loaded_scene = loaded_data.scene;

        // Verify sphere NOT present
        assert_eq!(loaded_scene.get_spheres().len(), 0);

        // Verify ray samples (should be default, which is 1)
        assert_ne!(loaded_scene.get_camera().get_ray_samples(), 10);

        // Verify hash color (should be default, which is true)
        assert_eq!(loaded_scene.get_color_hash_enabled(), true);
    }
}
