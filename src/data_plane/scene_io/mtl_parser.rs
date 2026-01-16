use crate::included_files::AutoPath;

#[derive(Debug)]
#[allow(dead_code)]
pub struct MTLParser {
    pub name: String,
    pub ka: Vec<f32>,
    pub kd: Vec<f32>,
    pub ks: Vec<f32>,
    pub ke: Vec<f32>,
    pub d: f32,
    pub ns: f32,
    pub illum: u32,
    pub map_kd: Option<String>,
    pub bump: Option<String>,
}

impl MTLParser {
    pub fn parse(path: AutoPath) -> anyhow::Result<Vec<MTLParser>> {
        let data = path.contents()?;
        if data.is_empty() {
            return Err(anyhow::Error::msg("MTL file is empty!"));
        }

        let mut return_mats = Vec::new();
        let mut name: String = String::with_capacity(2);
        let mut ka: Vec<f32> = Vec::with_capacity(10);
        let mut kd: Vec<f32> = Vec::with_capacity(10);
        let mut ks: Vec<f32> = Vec::with_capacity(10);
        let mut ke: Vec<f32> = Vec::with_capacity(10);
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
                        return_mats.push({
                            MTLParser {
                                name: name.clone(),
                                ka: ka.clone(),
                                kd: kd.clone(),
                                ks: ks.clone(),
                                ke: ke.clone(),
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
                    ke.clear();
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
                if line.starts_with("Ke") {
                    let temp = line.replace("Ke", "").trim().to_string();
                    let temp = temp.split_whitespace().collect::<Vec<&str>>();
                    for i in temp {
                        ke.push(i.parse::<f32>()?);
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
        return_mats.push({
            MTLParser {
                name: name.clone(),
                ka: ka.clone(),
                kd: kd.clone(),
                ks: ks.clone(),
                ke: ke.clone(),
                d,
                ns,
                illum,
                map_kd: map_kd.clone(),
                bump: bump.clone(),
            }
        });
        Ok(return_mats)
    }
}
