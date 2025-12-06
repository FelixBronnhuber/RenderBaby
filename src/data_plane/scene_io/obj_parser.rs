use std::fs;
use std::path::Path;

#[derive(Debug)]
pub struct FaceLine {
    pub v: Vec<u32>,
    pub vt: Vec<u32>,
    pub vn: Vec<u32>,
    pub material_name: String,
}
#[derive(Debug)]
#[allow(dead_code)]
pub struct OBJParser {
    pub vertices: Vec<f32>,                   //v
    pub face_indexes: Vec<FaceLine>,          //f
    pub normals: Option<Vec<f32>>,            //vn
    pub texture_coordinate: Option<Vec<f32>>, //vt
    pub material_path: Option<Vec<String>>,
}
impl OBJParser {
    #[allow(dead_code)]
    pub fn parse(path: &str) -> Option<OBJParser> {
        let path = Path::new(path);
        let data = fs::read_to_string(path).unwrap_or_default();
        if data.is_empty() {
            return None;
        }
        let lineiter = data.lines();

        let mut v_numarr = Vec::with_capacity(30);

        let mut vn_numarr = Vec::with_capacity(30);

        let mut vt_numarr = Vec::with_capacity(30);

        let mut facearr = Vec::with_capacity(100);
        let mut mtl_path: Vec<String> = Vec::with_capacity(2);

        let mut currentmaterial = String::new();
        lineiter.for_each(|l| {
            match l.split_once(" ") {
                Some(("vn", vn)) => {
                    let vec = vn.split_whitespace().collect::<Vec<&str>>();
                    vn_numarr.push(vec[0].parse::<f32>().unwrap_or_default());
                    vn_numarr.push(vec[1].parse::<f32>().unwrap_or_default());
                    vn_numarr.push(vec[2].parse::<f32>().unwrap_or_default());
                }
                Some(("vt", vt)) => {
                    let vec = vt.split_whitespace().collect::<Vec<&str>>();
                    vt_numarr.push(vec[0].parse::<f32>().unwrap_or_default());
                    vt_numarr.push(vec[1].parse::<f32>().unwrap_or_default());
                }
                Some(("v", v)) => {
                    let vec = v.split_whitespace().collect::<Vec<&str>>();
                    vec.iter()
                        .for_each(|a| v_numarr.push(a.parse::<(f32)>().unwrap_or_default()));
                }
                Some(("f", f)) => {
                    let f = f.trim();
                    let mut face = FaceLine {
                        v: Vec::with_capacity(50),
                        vt: Vec::with_capacity(10),
                        vn: Vec::with_capacity(10),
                        material_name: String::new(),
                    };
                    let mut com = f.split_whitespace().collect::<Vec<&str>>();
                    com.iter().for_each(|a| {
                        let mut split = a.split(|x| x == '/' || x == ' ');

                        let first = split.next().unwrap_or_default(); //v
                        let second = split.next().unwrap_or_default(); //vt
                        let third = split.next().unwrap_or_default(); //vn

                        face.v.push(first.parse::<u32>().unwrap());
                        face.vt.push(second.parse::<u32>().unwrap_or_default());
                        face.vn.push(third.parse::<u32>().unwrap_or_default());
                        loop {
                            let first = split.next().unwrap_or_default();
                            if first == "" {
                                break;
                            }
                            let second = split.next().unwrap_or_default(); //vt
                            let third = split.next().unwrap_or_default(); //vn

                            face.v.push(first.parse::<u32>().unwrap());
                            face.vt.push(second.parse::<u32>().unwrap());
                            face.vn.push(third.parse::<u32>().unwrap());
                        }
                    });
                    face.material_name = currentmaterial.clone();
                    facearr.push(face);
                }
                Some(("usemtl", usemtl)) => {
                    currentmaterial = usemtl.trim().to_string();
                }
                Some(("mtllib", mtllib)) => mtl_path.push(mtllib.trim().to_string()),

                _ => {}
            }
        });

        Some(OBJParser {
            vertices: v_numarr,
            face_indexes: facearr,
            material_path: if !mtl_path.is_empty() {
                Some(mtl_path)
            } else {
                None
            },
            normals: if !vn_numarr.is_empty() {
                Some(vn_numarr)
            } else {
                None
            },
            texture_coordinate: if !vt_numarr.is_empty() {
                Some(vt_numarr)
            } else {
                None
            },
        })
    }
}
