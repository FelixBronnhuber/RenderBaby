use crate::geometric_object::{Material, TriGeometry, Triangle};
use glam::Vec3;
use std::path::Path;
pub fn parseobj(obj_path: String) -> TriGeometry {
    let obj_path = Path::new(&obj_path);

    // Load the OBJ file
    let (models, materials) = tobj::load_obj(
        obj_path,
        &tobj::LoadOptions {
            triangulate: true,
            single_index: true,
            ..Default::default()
        },
    )
    .expect("error while trying to load obj file");

    let amount_models = models.len();
    let mut z: usize = 0;
    let mut return_triangles: Vec<Triangle> = Vec::new();
    models.iter().for_each(|model| {
        z = 0;
        let triangles = TriGeometry::new(Vec::new(), Material::default());
        let mut vec: Vec<Vec3> = Vec::new();
        for i in 0..3 {
            while z < (model.mesh.positions.len() / 3) {
                let point = (
                    model.mesh.positions[z * 3],
                    model.mesh.positions[z * 3 + 1],
                    model.mesh.positions[z * 3 + 2],
                );
                vec.push(point.into());
                z += 1;
            }
        }

        let i = model.mesh.indices.len() / 3;

        for u in 0..i {
            let mut a = Triangle::new(Vec::new(), None);
            a.add_point(vec[model.mesh.indices[u * 3] as usize]);
            a.add_point(vec[model.mesh.indices[u * 3 + 1] as usize]);
            a.add_point(vec[model.mesh.indices[u * 3 + 2] as usize]);
            return_triangles.push(a);
        }
    });

    let mats: &tobj::Material;
    let mut mat: Material = Material::default();
    
    if let material = materials.unwrap() {
        mats = material.first().unwrap();
        mat = Material::new(
            vec![
                mats.ambient.unwrap()[0].into(),
                mats.ambient.unwrap()[1].into(),
                mats.ambient.unwrap()[2].into(),
            ],
            vec![
                mats.diffuse.unwrap()[0].into(),
                mats.diffuse.unwrap()[1].into(),
                mats.diffuse.unwrap()[2].into(),
            ],
            vec![
                mats.specular.unwrap()[0].into(),
                mats.specular.unwrap()[1].into(),
                mats.specular.unwrap()[2].into(),
            ],
            mats.shininess.unwrap().into(),
            mats.dissolve.unwrap().into(),
        );
    }

    TriGeometry::new(return_triangles, mat)
}
