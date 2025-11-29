use glam::Vec3;

#[allow(dead_code)]
pub struct LightSource {
    position: Vec3,
    luminosity: f32,
    name: String,
    color: [f32; 3],
    rotation: Vec3,
    light_type: LightType,
}
#[allow(dead_code)]
impl LightSource {
    pub fn get_position(&self) -> Vec3 {
        self.position
    }

    pub fn set_position(&mut self, position: Vec3) {
        self.position = position
    }

    pub fn get_luminositoy(&self) -> f32 {
        self.luminosity
    }

    pub fn set_luminosity(&mut self, luminosity: f32) {
        self.luminosity = luminosity
    }

    pub fn get_rotation(&self) -> Vec3 {
        self.rotation
    }
    pub fn rotate(&mut self, _vec: Vec3) -> Vec3 {
        todo!()
        // rotate and return new orientation?
    }
    pub fn get_light_type(&self) -> &LightType {
        &self.light_type
    }
    pub fn get_color(&self) -> [f32; 3] {
        self.color
    }
    pub fn set_color(&mut self, color: [f32; 3]) {
        self.color = color;
    }
    pub fn get_name(&self) -> &String {
        &self.name
    }
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
    // todo LightSource should also be a FileObject!
    pub fn new(
        position: Vec3,
        luminosity: f32,
        color: [f32; 3],
        name: String,
        rotation: Vec3,
        light_type: LightType,
    ) -> Self {
        LightSource {
            position,
            luminosity,
            name,
            color,
            rotation, // some types have no ratation
            light_type,
        }
    }
}

#[allow(dead_code)]
pub enum LightType {
    Ambient,
    Point,
    Directional,
}
