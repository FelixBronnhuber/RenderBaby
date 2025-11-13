use std::path::Path;
//use super::lib;
use crate::geometric_object::{TriGeometry, Triangle};
use glam::Vec3;
pub fn parseobj() {
    let obj_path = Path::new("./test.obj");

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

    //models.iter().for_each(|model| { println!("{:?}{:?}", model.mesh.positions, model.mesh.indices); });
    let amount_models = models.len();
    //let model = models.first().unwrap();
    println!("model amount: {}", amount_models);
    println!("{:?}", models);
    //model.mesh.indices.iter().for_each(|index| {println!("{}", index+1);});
    //let mut z = model.mesh.indices[0] as usize;
    let mut z: usize = 0;
    models.iter().for_each(|model| {
        z = 0;
        let triangles = TriGeometry::new(Vec::new());
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
                println!("point: {:?}", point);
            }
        }
        println!("vec: {:?}", vec);
        println!("indices: {:?}", model.mesh.indices);
        let i = model.mesh.indices.len() / 3;

        for u in 0..i {
            let mut a = Triangle::new(Vec::new(), None);
            a.add_point(vec[model.mesh.indices[u * 3] as usize]);
            a.add_point(vec[model.mesh.indices[u * 3 + 1] as usize]);
            a.add_point(vec[model.mesh.indices[u * 3 + 2] as usize]);
            println!("triangle: {:?}", a.get_points());
        }

        //let a = Triangle::new(vec![Vec3::new(model.mesh.positions[z * 3], model.mesh.positions[z * 3 + 1], model.mesh.positions[z * 3 + 2])], None);
    })
    //

    /*
    for i in model.mesh.indices.iter(){
        println!("indiceloop {},{}",i,z);
        Triangle::new(vec![Vec3::new(model.mesh.positions[z * 3], model.mesh.positions[z * 3 + 1], model.mesh.positions[z * 3 + 2])],None);
        z = z+3;
    }*/
    /*
    let a = GeometricObject::Triangles(vec![
        for i in model.mesh.indices.iter().step_by(3) {
            let b = Triangle::new(vec![
                { Vec3::new(model.mesh.positions[i * 3], model.mesh.positions[i * 3 + 1], model.mesh.positions[i * 3 + 2]) }
            ], None)
        }]);
    match a { GeometricObject::Triangles(v) => {v.iter().for_each(|v|println!("{:?}",v.get_points()));}
                GeometricObject::Circle(c) => {}}
                */
    //let a = GeometricObject::Triangles(Triangle)//will vector of triangles

    /*let a = Vec3 {
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
    */
}
