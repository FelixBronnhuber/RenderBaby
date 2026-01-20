pub mod camera;
pub mod geometric_object;
pub mod light_source;
pub mod material;
pub mod mesh;
pub mod sphere;

#[cfg(test)]
mod tests {

    use glam::Vec3;

    use crate::{
        geometric_object::{GeometricObject, SceneObject},
        material::Material,
        mesh::Mesh,
        sphere::Sphere,
    };

    #[test]
    fn sphere_test() {
        let start_radius = 1.0;
        let scale_factor = 2.5;
        let mut sphere = Sphere::new(
            Vec3::default(),
            start_radius,
            Material::default(),
            [1.0, 0.0, 0.0],
        );
        sphere.scale(scale_factor);
        assert_eq!(sphere.get_radius(), start_radius * scale_factor);

        let translation = Vec3::new(10.0, 10.0, 10.0);
        sphere.translate(translation);
        assert_eq!(sphere.get_center(), translation);
        assert_eq!(sphere.get_translation(), translation);
    }
    #[test]
    fn mesh_test() {
        let mut v = vec![
            1.0, 1.0, 1.0, 2.0, 1.0, 1.0, 2.0, 2.0, 1.0, 1.0, 2.0, 1.0, 1.0, 1.0, 2.0, 2.0, 1.0,
            2.0, 2.0, 2.0, 2.0, 1.0, 2.0, 2.0,
        ];
        let mesh_res = Mesh::new(
            v.clone(),
            vec![
                1, 2, 3, 1, 3, 4, 5, 6, 7, 5, 7, 8, 1, 2, 6, 1, 6, 5, 2, 3, 7, 2, 7, 6, 3, 4, 8, 3,
                8, 7, 4, 1, 5, 4, 5, 8,
            ],
            None,
            None,
            None,
            None,
            None,
        );
        //assert_matches!(mesh, Ok(_));
        match mesh_res {
            Ok(mut mesh) => {
                assert_eq!(
                    Mesh::calculate_centroid(mesh.get_vertices()).unwrap(),
                    Vec3::new(1.5, 1.5, 1.5)
                );
                let translation = Vec3::new(-1.5, -1.5, -1.5);
                mesh.translate(translation);
                v.iter_mut().for_each(|x| *x += -1.5);
                assert_eq!(
                    Mesh::calculate_centroid(mesh.get_vertices()).unwrap(),
                    Vec3::default()
                );
                assert_eq!(*mesh.get_vertices(), v);

                let scale_factor = 2.0;
                mesh.scale(scale_factor);
                v.iter_mut().for_each(|x| *x *= scale_factor);
                assert_eq!(*mesh.get_vertices(), v);

                // todo: rotation test
            }
            Err(_) => panic!("Failed to create new mesh"),
        }
    }
}
