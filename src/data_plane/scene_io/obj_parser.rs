use std::ffi::OsStr;
use std::fs;
use std::path::PathBuf;
use crate::included_files::AutoPath;

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
    pub fn parse(path: AutoPath) -> Result<OBJParser, OBJParseError> {
        let data = match path.contents() {
            Ok(data) => data,
            Err(_) => {
                return Err(OBJParseError::FileRead(
                    "could not read file contents".to_string(),
                ));
            }
        };
        let directory_path = path.get_popped().unwrap();

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
                Some(("mtllib", mtllib)) => {
                    mtl_path.push(match directory_path.get_joined(mtllib.trim()) {
                        Some(path) => path.path_buf().to_string_lossy().to_string(),
                        None => String::new(),
                    })
                }

                _ => {}
            }
        }

        let filename = path
            .path()
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
