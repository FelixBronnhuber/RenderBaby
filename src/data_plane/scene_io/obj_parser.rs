use std::fs;
use std::path::Path;

#[derive(Debug)]
#[allow(dead_code)]
pub struct OBJParser {
    pub vertices: Vec<f32>, //points
    pub faces: Vec<u32>,    //faces[0] has id face_material_id[0] with the name material_id_name[0]
    pub face_material_id: Option<Vec<usize>>,
    pub normals: Option<Vec<f32>>,            //vn
    pub texture_coordinate: Option<Vec<f32>>, //vt
    pub material_path: Option<Vec<String>>,
    pub material_id_name: Option<Vec<String>>, //material_id_name of face
}
impl OBJParser {
    #[allow(dead_code)]
    pub fn parse(path: String) -> Option<OBJParser> {
        let path = Path::new(path.as_str());
        let data = fs::read_to_string(path).unwrap_or_default();

        let lineiter = data.lines();

        let mut v_vec: Vec<String> = Vec::with_capacity(30);
        let mut v_slicearr: Vec<Vec<&str>> = Vec::with_capacity(30);
        let mut v_numarr = Vec::with_capacity(30);

        let mut vn_vec: Vec<String> = Vec::with_capacity(30);
        let mut vn_slicearr: Vec<Vec<&str>> = Vec::with_capacity(30);
        let mut vn_numarr = Vec::with_capacity(30);

        let mut vt_vec: Vec<String> = Vec::with_capacity(30);
        let mut vt_slicearr: Vec<Vec<&str>> = Vec::with_capacity(30);
        let mut vt_numarr = Vec::with_capacity(30);

        let mut f_vec: Vec<String> = Vec::with_capacity(30);
        let mut f_id = Vec::with_capacity(30);
        let mut f_slicearr: Vec<Vec<&str>> = Vec::with_capacity(30);
        let mut f_numarr = Vec::with_capacity(30);

        let mut mtl_vec: Vec<String> = Vec::with_capacity(30);
        let mut mtl_path: Vec<String> = Vec::with_capacity(30);

        let mut material_id_name = Vec::with_capacity(1);
        material_id_name.push("NoMaterial".to_string());
        let mut id = 0;
        lineiter.for_each(|l| {
            if l.starts_with("v ") {
                v_vec.push(l.to_string());
            } else if l.starts_with("f") {
                f_vec.push(l.to_string());
                f_id.push(id);
            } else if l.starts_with("mtllib") {
                mtl_vec.push(l.to_string());
            } else if l.starts_with("usemtl") {
                material_id_name.push(l.to_string().replace("usemtl", "").trim().to_owned());
                id += 1
            } else if l.starts_with("vn") {
                vn_vec.push(l.to_string());
            } else if l.starts_with("vt") {
                vt_vec.push(l.to_string());
            }
        });

        for s in &mut v_vec {
            s.remove(0);
            let str = s.trim();
            let str = str.split_whitespace().collect::<Vec<&str>>();
            v_slicearr.push(str);
        }

        v_slicearr.iter().for_each(|a| {
            a.iter()
                .for_each(|s| v_numarr.push(s.parse::<f32>().unwrap()))
        });

        for s in &mut vn_vec {
            s.remove(0);
            s.remove(0);
            let str = s.trim();
            let str = str.split_whitespace().collect::<Vec<&str>>();
            vn_slicearr.push(str);
        }

        vn_slicearr.iter().for_each(|a| {
            a.iter()
                .for_each(|s| vn_numarr.push(s.parse::<f32>().unwrap()))
        });

        for s in &mut vt_vec {
            s.remove(0);
            s.remove(0);
            let str = s.trim();
            let str = str.split_whitespace().collect::<Vec<&str>>();
            vt_slicearr.push(str);
        }

        vt_slicearr.iter().for_each(|a| {
            a.iter()
                .for_each(|s| vt_numarr.push(s.parse::<f32>().unwrap()))
        });

        for s in &mut f_vec {
            s.remove(0);
            let str = s.trim();
            let mut str = str.split_whitespace().collect::<Vec<&str>>();
            if !str.is_empty() && str.first().unwrap().contains("/") {
                str = str
                    .iter()
                    .map(|line| line.split_once("/").map(|(str, _)| str).unwrap())
                    .collect();
            }

            f_slicearr.push(str);
        }

        f_slicearr.iter().for_each(|a| {
            a.iter()
                .for_each(|s| f_numarr.push(s.parse::<u32>().unwrap()))
        });

        for s in &mut mtl_vec {
            let str = s.split_whitespace().collect::<Vec<&str>>();
            mtl_path.push(str[1].to_string());
        }

        Some(OBJParser {
            vertices: v_numarr,
            faces: f_numarr,
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
            face_material_id: if !f_id.is_empty() { Some(f_id) } else { None },
            texture_coordinate: if !vt_numarr.is_empty() {
                Some(vt_numarr)
            } else {
                None
            },
            material_id_name: if !material_id_name.is_empty() {
                Some(material_id_name)
            } else {
                None
            },
        })
    }
}
