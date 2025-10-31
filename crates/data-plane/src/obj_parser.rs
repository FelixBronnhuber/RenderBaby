use std::path::Path;
//use super::lib;
//use crate::Position;
use glam::Vec3;
pub fn parseobj() {
    let obj_path = Path::new("./test.obj");

    // Load the OBJ file
    let (models, materials) = tobj::load_obj(obj_path,
                                             &tobj::LoadOptions {
                                                 triangulate: true,
                                                 single_index: true,
                                                 ..Default::default()
                                             },
    ).expect("error while trying to load obj file");

    models.iter().for_each(|model| { println!("{:?}{:?}", model.mesh.positions, model.mesh.indices); });
    let model = models.first().unwrap();
    let i = model.mesh.indices[0] as usize;
    let a = Vec3 {
        x: model.mesh.positions[i*3],
        y: model.mesh.positions[i*3 + 1],
        z: model.mesh.positions[i*3 + 2]
    };
    println!("{},{},{}",a.x,a.y,a.z);
    let b = Vec3 {
        x: model.mesh.positions[i*3+3],
        y: model.mesh.positions[i*3 + 4],
        z: model.mesh.positions[i*3 + 5]
    };
    println!("{},{},{}",b.x,b.y,b.z);
    let a = Vec3::new(1.0, 0.0, 0.0);
}

