use std::ffi::OsStr;
use log::error;
use scene_objects::material::Material;
use scene_objects::mesh::Mesh;
use crate::data_plane::scene_io::mtl_parser::load_mtl;
use crate::data_plane::scene_io::texture_loader::TextureCache;
use crate::included_files::AutoPath;

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
    pub fn parse(path: AutoPath) -> anyhow::Result<OBJParser> {
        let data = path.contents()?;
        let directory_path = path.get_popped().unwrap();

        if data.is_empty() {
            return Err(anyhow::Error::msg("OBJ file is empty!"));
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
                Some(("mtllib", mtllib)) => match directory_path.get_joined(mtllib.trim()) {
                    Some(path) => mtl_path.push(path.path_buf().to_string_lossy().to_string()),
                    None => error!("Could not find mtllib file: {}", mtllib.trim()),
                },

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

pub struct ObjLoadResult {
    pub mesh: Mesh,
}

pub fn load_obj(
    auto_path: AutoPath,
    texture_cache: &mut TextureCache,
) -> anyhow::Result<ObjLoadResult> {
    let objs = OBJParser::parse(auto_path.clone())?;

    let mut materials: Vec<Material> = Vec::new();
    let mut material_name_list: Vec<String> = Vec::new();
    let parent_dir = auto_path.get_popped().unwrap();

    if let Some(mtl_paths) = objs.material_path.clone() {
        for rel in mtl_paths {
            let mtl_path = AutoPath::get_absolute_or_join(&rel, &parent_dir)?;
            match load_mtl(mtl_path.clone()) {
                Ok(mats) => {
                    material_name_list.extend(mats.iter().map(|m| m.name.clone()));
                    materials.extend(mats.into_iter());
                }
                Err(_e) => {}
            }
        }
    }

    for m in &materials {
        #[allow(clippy::collapsible_if)]
        if let Some(tex_path_str) = &m.texture_path {
            if let Ok(ap) = AutoPath::try_from(tex_path_str.clone()) {
                let _ = texture_cache.load(ap);
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
                let v_idx = face.v[idx] as usize;
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
        Some(auto_path.path_buf()),
    )?;

    Ok(ObjLoadResult { mesh })
}
