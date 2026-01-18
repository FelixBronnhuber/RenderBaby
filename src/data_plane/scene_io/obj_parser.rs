use std::ffi::OsStr;
use std::fs;
use std::path::PathBuf;
use scene_objects::material::Material;
use scene_objects::mesh::Mesh;
use crate::data_plane::scene_io::texture_loader::TextureCache;
use crate::data_plane::scene_io::mtl_parser::load_mtl;
use std::path::Path;

#[derive(Debug)]
pub enum OBJParseError {
    Path(std::io::Error),
    FileRead(String),
    ParseInteger(std::num::ParseIntError),
    ParseFloat(std::num::ParseFloatError),
}

impl std::fmt::Display for OBJParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OBJParseError::Path(error) => write!(f, "invalid file path: {}", error),
            OBJParseError::FileRead(e) => write!(f, "file read error: {}", e),
            OBJParseError::ParseInteger(e) => write!(f, "invalid integer error: {}", e),
            OBJParseError::ParseFloat(e) => write!(f, "invalid float error: {}", e),
        }
    }
}
impl std::error::Error for OBJParseError {}
impl From<std::io::Error> for OBJParseError {
    fn from(e: std::io::Error) -> Self {
        OBJParseError::Path(e)
    }
}
impl From<std::num::ParseIntError> for OBJParseError {
    fn from(e: std::num::ParseIntError) -> Self {
        OBJParseError::ParseInteger(e)
    }
}

impl From<std::num::ParseFloatError> for OBJParseError {
    fn from(e: std::num::ParseFloatError) -> Self {
        OBJParseError::ParseFloat(e)
    }
}

#[derive(Debug)]
pub struct FaceLine {
    pub v: Vec<f32>,
    pub vt: Vec<f32>,
    pub vn: Vec<f32>,
    pub material_name: String,
}
#[derive(Debug)]
#[allow(dead_code)]
pub struct OBJParser {
    pub name: String,
    pub vertices: Vec<f32>,                   //v
    pub faces: Vec<FaceLine>,                 //f
    pub normals: Option<Vec<f32>>,            //vn
    pub texture_coordinate: Option<Vec<f32>>, //vt
    pub material_path: Option<Vec<String>>,   // consider using Vec<PathBuf> instead!
}
impl OBJParser {
    #[allow(dead_code)]
    pub fn parse(path: PathBuf) -> Result<OBJParser, OBJParseError> {
        let path_clone = path.clone();
        let data = fs::read_to_string(path_clone.clone())?;
        let mut directory_path = path_clone;
        directory_path.pop();
        if data.is_empty() {
            return Err(OBJParseError::FileRead("empty file".to_string()));
        }

        let mut v_numarr = Vec::with_capacity(30);

        let mut vn_numarr = Vec::with_capacity(30);

        let mut vt_numarr = Vec::with_capacity(30);

        let mut facearr = Vec::with_capacity(100);
        let mut mtl_path: Vec<String> = Vec::with_capacity(2);

        let mut currentmaterial = String::new();

        for l in data.lines() {
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
                    vec.iter().for_each(|a| v_numarr.push(a.parse::<f32>()));
                }
                Some(("f", f)) => {
                    let f = f.trim();
                    let mut face = FaceLine {
                        v: Vec::with_capacity(50),
                        vt: Vec::with_capacity(10),
                        vn: Vec::with_capacity(10),
                        material_name: String::new(),
                    };
                    let com = f.split_whitespace().collect::<Vec<&str>>();
                    com.iter().for_each(|a| {
                        let mut split = a.split(['/', ' ']);
                        let first = split.next().unwrap_or_default(); //v
                        let second = split.next().unwrap_or_default(); //vt
                        let third = split.next().unwrap_or_default(); //vn

                        face.v.push(first.parse::<f32>().unwrap_or_default());
                        face.vt.push(second.parse::<f32>().unwrap_or_default());
                        face.vn.push(third.parse::<f32>().unwrap_or_default());
                        loop {
                            let first = split.next().unwrap_or_default();
                            if first.is_empty() {
                                break;
                            }
                            let second = split.next().unwrap_or_default(); //vt
                            let third = split.next().unwrap_or_default(); //vn

                            face.v.push(first.parse::<f32>().unwrap());
                            face.vt.push(second.parse::<f32>().unwrap());
                            face.vn.push(third.parse::<f32>().unwrap());
                        }
                    });
                    face.material_name = currentmaterial.clone();
                    facearr.push(face);
                }
                Some(("usemtl", usemtl)) => {
                    currentmaterial = usemtl.trim().to_string();
                }
                Some(("mtllib", mtllib)) => mtl_path.push(
                    directory_path
                        .join(mtllib.trim())
                        .to_string_lossy()
                        .to_string(),
                ),

                _ => {}
            }
        }

        let filename = path
            .file_name()
            .unwrap_or(OsStr::new(" "))
            .to_string_lossy()
            .to_string();
        Ok(OBJParser {
            name: filename,
            vertices: v_numarr.into_iter().collect::<Result<Vec<_>, _>>()?,
            faces: facearr,
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

pub struct ObjLoadResult {
    pub mesh: Mesh,
    pub materials: Vec<Material>,
}

pub fn load_obj(path: PathBuf, texture_cache: &mut TextureCache) -> anyhow::Result<ObjLoadResult> {
    let objs = OBJParser::parse(path.clone())?;

    let mut materials: Vec<Material> = Vec::new();
    let mut material_name_list: Vec<String> = Vec::new();
    let parent_dir = Path::new(&path)
        .parent()
        .map(|p| p.to_path_buf())
        .unwrap_or(Path::new(".").to_path_buf());

    if let Some(mtl_paths) = objs.material_path.clone() {
        for rel in mtl_paths {
            let mtl_path = parent_dir.join(&rel);
            match load_mtl(mtl_path.clone()) {
                Ok(mut mats) => {
                    for m in &mut mats {
                        if let Some(tex) = &m.texture_path {
                            let _ = texture_cache.load(tex);
                        }
                    }
                    material_name_list.extend(mats.iter().map(|m| m.name.clone()));
                    materials.extend(mats.into_iter());
                }
                Err(_e) => {}
            }
        }
    }

    let mut new_vertices = Vec::with_capacity(objs.faces.len() * 9);
    let mut new_tris = Vec::with_capacity(objs.faces.len() * 3);
    let mut new_uvs = Vec::with_capacity(objs.faces.len() * 6);
    let mut material_index = Vec::with_capacity(objs.faces.len());

    let mut vertex_count: u32 = 0;
    for face in objs.faces {
        let mat_idx = material_name_list
            .iter()
            .position(|n| n == &face.material_name)
            .unwrap_or(0);

        let leng = face.v.len();
        for i in 1..(leng - 1) {
            let v_indices = [0usize, i, i + 1];

            for &idx in &v_indices {
                let v_idx = face.v[idx] as usize - 1;
                if v_idx * 3 + 2 < objs.vertices.len() {
                    new_vertices.push(objs.vertices[v_idx * 3]);
                    new_vertices.push(objs.vertices[v_idx * 3 + 1]);
                    new_vertices.push(objs.vertices[v_idx * 3 + 2]);
                } else {
                    new_vertices.extend_from_slice(&[0.0, 0.0, 0.0]);
                }

                if !face.vt.is_empty() && idx < face.vt.len() {
                    let vt_val = face.vt[idx] as usize;
                    if vt_val > 0 {
                        let vt_idx = vt_val - 1;
                        if let Some(tex_coords) = &objs.texture_coordinate {
                            if vt_idx * 2 + 1 < tex_coords.len() {
                                new_uvs.push(tex_coords[vt_idx * 2]);
                                new_uvs.push(tex_coords[vt_idx * 2 + 1]);
                            } else {
                                new_uvs.extend_from_slice(&[0.0, 0.0]);
                            }
                        } else {
                            new_uvs.extend_from_slice(&[0.0, 0.0]);
                        }
                    } else {
                        new_uvs.extend_from_slice(&[0.0, 0.0]);
                    }
                } else {
                    new_uvs.extend_from_slice(&[0.0, 0.0]);
                }

                new_tris.push(vertex_count);
                vertex_count += 1;
            }
            material_index.push(mat_idx);
        }
    }

    let mesh = Mesh::new(
        new_vertices,
        new_tris,
        if new_uvs.is_empty() {
            None
        } else {
            Some(new_uvs)
        },
        if materials.is_empty() {
            None
        } else {
            Some(materials.clone())
        },
        if material_index.is_empty() {
            None
        } else {
            Some(material_index)
        },
        Some(objs.name.clone()),
        Some(path.clone()),
    )?;

    Ok(ObjLoadResult { mesh, materials })
}
