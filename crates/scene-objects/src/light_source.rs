use glam::Vec3;
/// Defines light sources for the scene
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct LightSource {
    position: Vec3,
    luminosity: f32,
    pub name: String,
    pub color: [f32; 3],
    pub rotation: Vec3,
    // pub light_type: LightType,
}
#[allow(dead_code)]
impl LightSource {
    /// ## Returns
    /// Position of the LightSource as glam::Vec3
    pub fn get_position(&self) -> Vec3 {
        self.position
    }
    /// Sets the position of the LightSource
    /// ## Params
    /// 'position': New position as glam::Vec3
    pub fn set_position(&mut self, position: Vec3) {
        self.position = position
    }
    /// ## Returns
    /// LightSource luminositity as f32 representing luminosity in watt
    pub fn get_luminositoy(&self) -> f32 {
        self.luminosity
    }
    /// Sets the luminosity
    /// ## Parameter
    /// luminosity: New luminosity as f32 representing luminosity in watt
    pub fn set_luminosity(&mut self, luminosity: f32) {
        self.luminosity = luminosity
    }
    /// ## Returns
    /// Rotation as glam::Vec3
    pub fn get_rotation(&self) -> Vec3 {
        self.rotation
    }
    /// Rotates by given vector
    /// ## Parameter
    /// 'vec': Rotation vector
    /// ## Returns:
    /// New Rotation as glam::Vec3
    pub fn rotate(&mut self, _vec: Vec3) -> Vec3 {
        todo!()
    }
    /// ## Returns
    /// LightSource color as rgb array of f32, values in \[0, 1]
    pub fn get_color(&self) -> [f32; 3] {
        self.color
    }
    /// Sets the LightSource color
    /// ## Parameter
    /// 'color': New LightSource color as array of f32, values in \[0, 1]
    pub fn set_color(&mut self, color: [f32; 3]) {
        self.color = color;
    }
    /// ## Returns
    /// Reference to the name
    pub fn get_name(&self) -> &String {
        &self.name
    }
    /// Sets the name of the LightSource
    /// ## Parameters
    /// 'name': new name
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
    /// Constructor for LightSource
    pub fn new(
        position: Vec3,
        luminosity: f32,
        color: [f32; 3],
        name: String,
        rotation: Vec3,
        //light_type: LightType,
    ) -> Self {
        LightSource {
            position,
            luminosity,
            name,
            color,
            rotation, // some types have no ratation
                      //light_type,
        }
    }
}

impl std::fmt::Display for LightSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "LightSource {} at {}",
            //self.get_light_type(),
            self.name,
            self.get_position()
        )
    }
}
