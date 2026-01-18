use std::cmp::PartialEq;
use std::sync::LazyLock;
use anyhow::anyhow;

static PLASTIC: LazyLock<Material> = LazyLock::new(|| Material::from(MaterialPresets::Plastic));
static METAL: LazyLock<Material> = LazyLock::new(|| Material::from(MaterialPresets::Metal));
static MIRROR: LazyLock<Material> = LazyLock::new(|| Material::from(MaterialPresets::Mirror));
static LIGHT: LazyLock<Material> = LazyLock::new(|| Material::from(MaterialPresets::Light));

#[derive(Debug, PartialEq)]
pub struct Material {
    pub name: String,
    pub ambient_reflectivity: Vec<f64>,  //Ka
    pub diffuse_reflectivity: Vec<f64>,  //Kd
    pub specular_reflectivity: Vec<f64>, //Ks
    pub emissive: Vec<f64>,              //Ke
    pub shininess: f64,                  //Ns
    pub transparency: f64,               //d
    pub texture_path: Option<String>,    //map_Kd
    pub ref_path: Option<String>,
}

#[allow(dead_code)]
impl Material {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        name: String,
        ambient_reflectivity: Vec<f64>,
        diffuse_reflectivity: Vec<f64>,
        specular_reflectivity: Vec<f64>,
        emissive: Vec<f64>,
        shininess: f64,
        transparency: f64,
        texture_path: Option<String>,
        ref_path: Option<String>,
    ) -> Self {
        Material {
            name,
            ambient_reflectivity,
            diffuse_reflectivity,
            specular_reflectivity,
            emissive,
            shininess,
            transparency,
            texture_path,
            ref_path,
        }
    }
}

impl Clone for Material {
    fn clone(&self) -> Material {
        Material {
            name: self.name.clone(),
            ambient_reflectivity: self.ambient_reflectivity.clone(),
            diffuse_reflectivity: self.diffuse_reflectivity.clone(),
            specular_reflectivity: self.specular_reflectivity.clone(),
            emissive: self.emissive.clone(),
            shininess: self.shininess,
            transparency: self.transparency,
            texture_path: self.texture_path.clone(),
            ref_path: self.ref_path.clone(),
        }
    }
}

impl Default for Material {
    fn default() -> Self {
        MaterialPresets::default().into()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum MaterialPresets {
    Plastic,
    Metal,
    #[default]
    Mirror,
    Light,
}

impl MaterialPresets {
    pub const fn list() -> [&'static str; 4] {
        ["plastic", "metal", "mirror", "light"]
    }

    pub const fn list_enum() -> [MaterialPresets; 4] {
        [
            MaterialPresets::Plastic,
            MaterialPresets::Metal,
            MaterialPresets::Mirror,
            MaterialPresets::Light,
        ]
    }
}

impl From<MaterialPresets> for String {
    fn from(preset: MaterialPresets) -> Self {
        match preset {
            MaterialPresets::Plastic => "plastic",
            MaterialPresets::Metal => "metal",
            MaterialPresets::Mirror => "mirror",
            MaterialPresets::Light => "light",
        }
        .to_string()
    }
}

impl TryFrom<&str> for MaterialPresets {
    type Error = anyhow::Error;

    fn try_from(string: &str) -> Result<Self, Self::Error> {
        match string {
            "plastic" => Ok(MaterialPresets::Plastic),
            "metal" => Ok(MaterialPresets::Metal),
            "mirror" => Ok(MaterialPresets::Mirror),
            "light" => Ok(MaterialPresets::Light),
            _ => Err(anyhow!("Invalid material preset: {}", string)),
        }
    }
}

impl From<MaterialPresets> for Material {
    fn from(preset: MaterialPresets) -> Self {
        match preset {
            MaterialPresets::Plastic => Material::new(
                "plastic".to_string(),
                vec![0.0, 0.0, 0.0],
                vec![1.0, 1.0, 1.0],
                vec![0.0, 0.0, 0.0],
                vec![0.0, 0.0, 0.0],
                0.0,
                1.0,
                None,
                None,
            ),
            MaterialPresets::Light => Material::new(
                "light".to_string(),
                vec![0.0, 0.0, 0.0],
                vec![0.0, 0.0, 0.0],
                vec![0.0, 0.0, 0.0],
                vec![100.0, 100.0, 100.0],
                0.0,
                1.0,
                None,
                None,
            ),
            MaterialPresets::Mirror => Material::new(
                "mirror".to_string(),
                vec![0.0, 0.0, 0.0],
                vec![0.0, 0.0, 0.0],
                vec![1.0, 1.0, 1.0],
                vec![0.0, 0.0, 0.0],
                1000.0,
                0.0,
                None,
                None,
            ),
            MaterialPresets::Metal => Material::new(
                "metal".to_string(),
                vec![0.0, 0.0, 0.0],
                vec![0.0, 0.0, 0.0],
                vec![0.5, 0.5, 0.5],
                vec![0.0, 0.0, 0.0],
                500.0,
                1.0,
                None,
                None,
            ),
        }
    }
}

impl TryFrom<&Material> for MaterialPresets {
    type Error = anyhow::Error;

    fn try_from(mtl: &Material) -> anyhow::Result<Self> {
        for preset in MaterialPresets::list_enum() {
            if *mtl == Material::from(preset) {
                return Ok(preset);
            }
        }
        Err(anyhow!("Material {:?} is not a preset", mtl))
    }
}

#[derive(Debug)]
pub enum MaterialRef {
    Preset(MaterialPresets),
    Custom(Material),
}

impl Default for MaterialRef {
    fn default() -> Self {
        MaterialRef::Preset(MaterialPresets::default())
    }
}

impl MaterialRef {
    pub fn get_material(&self) -> &Material {
        match self {
            MaterialRef::Preset(preset) => match preset {
                MaterialPresets::Plastic => &PLASTIC,
                MaterialPresets::Metal => &METAL,
                MaterialPresets::Mirror => &MIRROR,
                MaterialPresets::Light => &LIGHT,
            },
            MaterialRef::Custom(mtl) => mtl,
        }
    }
}

impl PartialEq<Material> for &Material {
    fn eq(&self, other: &Material) -> bool {
        self.name == other.name
            && self.ambient_reflectivity == other.ambient_reflectivity
            && self.diffuse_reflectivity == other.diffuse_reflectivity
            && self.specular_reflectivity == other.specular_reflectivity
            && self.emissive == other.emissive
            && self.shininess == other.shininess
            && self.transparency == other.transparency
            && self.texture_path == other.texture_path
    }
}

impl From<Material> for MaterialRef {
    fn from(mtl: Material) -> Self {
        for def_mtl_name in MaterialPresets::list() {
            let def_mtl = MaterialPresets::try_from(def_mtl_name).unwrap();
            if mtl == Material::from(def_mtl) {
                return MaterialRef::Preset(def_mtl);
            }
        }
        MaterialRef::Custom(mtl)
    }
}
