use std::fs;
#[derive(Debug)]
pub enum MTLParseError {
    Path(std::io::Error),
    FileRead(String),
    ParseInteger(std::num::ParseIntError),
    ParseFloat(std::num::ParseFloatError),
}
impl std::fmt::Display for MTLParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MTLParseError::Path(error) => write!(f, "invalid file path: {}", error),
            MTLParseError::FileRead(e) => write!(f, "file read error: {}", e),
            MTLParseError::ParseInteger(e) => write!(f, "invalid integer error: {}", e),
            MTLParseError::ParseFloat(e) => write!(f, "invalid float error: {}", e),
        }
    }
}
impl std::error::Error for MTLParseError {}
impl From<std::io::Error> for MTLParseError {
    fn from(e: std::io::Error) -> Self {
        MTLParseError::Path(e)
    }
}
impl From<std::num::ParseIntError> for MTLParseError {
    fn from(e: std::num::ParseIntError) -> Self {
        MTLParseError::ParseInteger(e)
    }
}
impl From<std::num::ParseFloatError> for MTLParseError {
    fn from(e: std::num::ParseFloatError) -> Self {
        MTLParseError::ParseFloat(e)
    }
}
#[derive(Debug)]
#[allow(dead_code)]
pub struct MTLParser {
    pub name: String,
    pub ka: Vec<f32>,
    pub kd: Vec<f32>,
    pub ks: Vec<f32>,
    pub d: f32, //transparency
    pub ns: f32,
    pub illum: u32,
    pub map_kd: Option<String>,
    pub bump: Option<String>,
}
impl MTLParser {
    pub fn parse(path: &str) -> Result<Vec<MTLParser>, MTLParseError> {
        let data = fs::read_to_string(path)?;
        if data.is_empty() {
            return Err(MTLParseError::FileRead("empty file".to_string()));
        }

        let mut returnmats = Vec::new();
        let mut name: String = String::with_capacity(2);
        let mut ka: Vec<f32> = Vec::with_capacity(10);
        let mut kd: Vec<f32> = Vec::with_capacity(10);
        let mut ks: Vec<f32> = Vec::with_capacity(10);
        let mut d: f32 = 0.0;
        let mut ns: f32 = 0.0;
        let mut illum: u32 = 0;
        let mut map_kd: Option<String> = None;
        let mut bump: Option<String> = None;

        let lineiter = data.lines();
        for l in lineiter {
            if !l.is_empty() && !l.starts_with('#') {
                let line = l.trim();
                if line.starts_with("newmtl") {
                    if !name.is_empty() {
                        returnmats.push({
                            MTLParser {
                                name: name.clone(),
                                ka: ka.clone(),
                                kd: kd.clone(),
                                ks: ks.clone(),
                                d,
                                ns,
                                illum,
                                map_kd: map_kd.clone(),
                                bump: bump.clone(),
                            }
                        });
                    }
                    name.clear();
                    ka.clear();
                    kd.clear();
                    ks.clear();
                    d = 0.0;
                    illum = 0;
                    ns = 0.0;
                    map_kd = None;
                    bump = None;
                    name = line.replace("newmtl", "").trim().to_string();
                }
                if line.starts_with("Ka") {
                    let temp = line.replace("Ka", "").trim().to_string();
                    let temp = temp.split_whitespace().collect::<Vec<&str>>();
                    for i in temp {
                        ka.push(i.parse::<f32>()?);
                    }
                }
                if line.starts_with("Kd") {
                    let temp = line.replace("Kd", "").trim().to_string();
                    let temp = temp.split_whitespace().collect::<Vec<&str>>();
                    for i in temp {
                        kd.push(i.parse::<f32>()?);
                    }
                }
                if line.starts_with("Ks") {
                    let temp = line.replace("Ks", "").trim().to_string();
                    let temp = temp.split_whitespace().collect::<Vec<&str>>();
                    for i in temp {
                        ks.push(i.parse::<f32>()?);
                    }
                }
                if line.starts_with("d") {
                    let temp = line.replace("d", "").trim().to_string();
                    let temp = temp.split_whitespace().collect::<Vec<&str>>();
                    for i in temp {
                        d = i.parse::<f32>()?;
                    }
                }
                if line.starts_with("Ns") {
                    let temp = line.replace("Ns", "").trim().to_string();
                    let temp = temp.split_whitespace().collect::<Vec<&str>>();
                    for i in temp {
                        ns = i.parse::<f32>()?;
                    }
                }
                if line.starts_with("illum") {
                    let temp = line.replace("illum", "").trim().to_string();
                    let temp = temp.split_whitespace().collect::<Vec<&str>>();
                    for i in temp {
                        illum = i.parse::<u32>()?;
                    }
                }
                if line.starts_with("map_Kd") {
                    let temp = line.replace("map_Kd", "").trim().to_string();
                    let temp = temp.split_whitespace().collect::<Vec<&str>>();
                    for i in temp {
                        map_kd = Some(i.to_string());
                    }
                }
                if line.starts_with("bump") {
                    let temp = line.replace("bump", "").trim().to_string();
                    let temp = temp.split_whitespace().collect::<Vec<&str>>();
                    for i in temp {
                        bump = Some(i.to_string());
                    }
                }
            }
        }
        returnmats.push({
            MTLParser {
                name: name.clone(),
                ka: ka.clone(),
                kd: kd.clone(),
                ks: ks.clone(),
                d,
                ns,
                illum,
                map_kd: map_kd.clone(),
                bump: bump.clone(),
            }
        });
        Ok(returnmats)
    }
}
