use glam::Vec3;
/// Camera that is used to render scenes
#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub struct Camera {
    position: Vec3,
    pane_distance: f32,
    pane_width: f32,
    pane_height: f32,
    look_at: Vec3,
    up: Vec3,
    resolution: Resolution,
    ray_samples: u32, // todo move to scene
}
#[allow(dead_code)]
impl Camera {
    /// sets the position of the camera
    /// ## Parameter
    /// 'position': glam::Vec3 of the new position
    pub fn set_position(&mut self, position: Vec3) {
        self.position = position;
    }

    pub fn set_look_at(&mut self, look_at: Vec3) {
        self.look_at = look_at
    }
    pub fn get_position(&self) -> Vec3 {
        self.position
    }
    /// ## Returns
    /// Camera position as glam::Vec3
    pub fn get_look_at(&self) -> Vec3 {
        self.look_at
    }
    /// ## Returns
    /// up vector of the camera
    pub fn get_up(&self) -> Vec3 {
        self.up
    }
    /// Sets the up vector of the camera to the given value
    /// ## Parameter
    /// 'up': glam::Vec3 for the new vector
    pub fn set_up(&mut self, up: Vec3) {
        self.up = up;
    }
    /// ## Returns
    /// Camera field of view, calculated from width and distance
    pub fn get_fov(&self) -> f32 {
        2.0 * (self.pane_width / (2.0 * self.pane_distance)).atan()
    }
    /// Set the camera pane distance
    /// ## Parameter
    /// distance: New value for pane_distance
    pub fn set_pane_distance(&mut self, distance: f32) {
        if distance >= 0.0 {
            self.pane_distance = distance;
        }
    }
    /// ## Returns
    /// Camera pane distance
    pub fn get_pane_distance(&self) -> f32 {
        self.pane_distance
    }
    /// Set the camera pane width
    /// ## Parameter
    /// width: New value for pane_distance
    pub fn set_pane_width(&mut self, width: f32) {
        if width > 0.0 {
            self.pane_width = width;
        }
    }
    /// ## Returns
    /// Camera pane width
    pub fn get_pane_width(&self) -> f32 {
        self.pane_width
    }
    /// ## Returns
    /// Camera resolution as Array of u32
    pub fn get_resolution(&self) -> &Resolution {
        &self.resolution
    }
    /// Sets the camera resolution
    /// ## Parameter
    /// 'resolution': New resolution as array of u32
    pub fn set_resolution(&mut self, resolution: Resolution) {
        self.resolution = resolution
    }
    /// Constructor for Camera
    /// ## Parameter
    /// 'position': Position of the new Camera as glam::Vec3
    /// 'rotation': Rotation of the new Camera as glam::Vec3
    /// # Returns
    /// A new camera with the given position and rotation
    pub fn new(position: Vec3, rotation: Vec3) -> Self {
        let mut res = Camera::default();
        res.set_position(position);
        res.set_look_at(rotation);
        res
    }
    /// ## Returns
    /// Camera ray samples
    pub fn get_ray_samples(&self) -> u32 {
        self.ray_samples
    }
    /// Sets the camera ray samples
    /// ## Parameter
    /// 'samples': new ray sample value
    pub fn set_ray_samples(&mut self, samples: u32) {
        self.ray_samples = samples;
        // here could be a check for values [1, 100] or so
    }
}

impl Default for Camera {
    fn default() -> Self {
        let resolution = Resolution::default();
        let pane_width = 36.0;
        let ratio = resolution.height as f32 / resolution.width as f32;
        let pane_height = pane_width * ratio;
        Self {
            position: Vec3::new(5.0, 5.0, 5.0),
            resolution,
            ray_samples: 1,
            look_at: Vec3::new(1.0, 1.0, 1.0),
            up: Vec3::new(0.0, 1.0, 0.0),
            pane_distance: 35.0,
            pane_width,
            pane_height,
        }
    }
}

impl std::fmt::Display for Camera {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Camera at {}", self.get_position())
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Resolution {
    pub width: u32,
    pub height: u32,
}
// todo: later add implementation for min, max, hd,uhd, ...
impl Resolution {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
}

impl Default for Resolution {
    fn default() -> Self {
        Self {
            width: 1920,
            height: 1080,
        }
    }
}
