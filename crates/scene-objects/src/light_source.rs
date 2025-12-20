use glam::Vec3;
use serde::{Deserialize, Serialize};
/// Defines light sources for the scene
#[allow(dead_code)]
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct LightSource {
    position: Vec3,
    luminosity: f32,
    name: String,
    color: [f32; 3],
    //#[serde(skip_serializing_if = "Vec3::is_zero")]
    rotation: Vec3,
    light_type: LightType,
}
#[allow(dead_code)]
impl LightSource {
    pub fn get_position(&self) -> Vec3 {
        //! ## Returns
        //! Position of the LightSource as glam::Vec3
        self.position
    }

    pub fn set_position(&mut self, position: Vec3) {
        //! Sets the position of the LightSource
        //! ## Params
        //! 'position': New position as glam::Vec3
        self.position = position
    }

    pub fn get_luminositoy(&self) -> f32 {
        //! ## Returns
        //! LightSource luminositity as f32 representing luminosity in watt
        self.luminosity
    }

    pub fn set_luminosity(&mut self, luminosity: f32) {
        //! Sets the luminosity
        //! ## Parameter
        //! luminosity: New luminosity as f32 representing luminosity in watt
        self.luminosity = luminosity
    }

    pub fn get_rotation(&self) -> Vec3 {
        //! ## Returns
        //! Rotation as glam::Vec3
        self.rotation
    }
    pub fn rotate(&mut self, _vec: Vec3) -> Vec3 {
        //! Rotates by given vector
        //! ## Parameter
        //! 'vec': Rotation vector
        //! ## Returns:
        //! New Rotation as glam::Vec3
        todo!()
    }
    pub fn get_light_type(&self) -> &LightType {
        //! ## Returns
        //! LightSource LightType Enum
        &self.light_type
    }
    pub fn get_color(&self) -> [f32; 3] {
        //! ## Returns
        //! LightSource color as rgb array of f32, values in \[0, 1]
        self.color
    }
    pub fn set_color(&mut self, color: [f32; 3]) {
        //! Sets the LightSource color
        //! ## Parameter
        //! 'color': New LightSource color as array of f32, values in \[0, 1]
        self.color = color;
    }
    pub fn get_name(&self) -> &String {
        //! ## Returns
        //! Reference to the name
        &self.name
    }
    pub fn set_name(&mut self, name: String) {
        //! Sets the name of the LightSource
        //! ## Parameters
        //! 'name': new name
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
        //! Constructor for LightSource
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

impl std::fmt::Display for LightSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?} LightSource {} at {}",
            self.get_light_type(),
            self.name,
            self.get_position()
        )
    }
}

#[allow(dead_code)]
#[derive(Clone, Copy, Serialize, Deserialize, Debug)]
/// Light can be either Ambient, Point or Directional
pub enum LightType {
    Ambient,
    Point,
    Directional,
}
