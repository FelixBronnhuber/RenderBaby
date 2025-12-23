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
    pub fn set_position(&mut self, position: Vec3) {
        //! sets the position of the camera
        //! ## Parameter
        //! 'position': glam::Vec3 of the new position
        self.position = position;
    }
    pub fn set_look_at(&mut self, look_at: Vec3) {
        //! sets the direction of the camera
        //! ## Parameter
        //! 'look_at': glam::Vec3 of the new direction
        self.look_at = look_at
    }
    pub fn get_position(&self) -> Vec3 {
        //! ## Returns
        //! Camera position as glam::Vec3
        self.position
    }
    pub fn get_look_at(&self) -> Vec3 {
        //! ## Returns
        //! Camera look at point as glam::Vec3
        self.look_at
    }
    pub fn get_up(&self) -> Vec3 {
        //! ## Returns
        //! up vector of the camera
        self.up
    }
    pub fn set_up(&mut self, up: Vec3) {
        //! Sets the up vector of the camera to the given value
        //! ## Parameter
        //! 'up': glam::Vec3 for the new vector
        self.up = up;
    }
    pub fn get_fov(&self) -> f32 {
        //! ## Returns
        //! Camera field of view, calculated drom width and distance
        //self.fov
        2.0 * (self.pane_width / (2.0 * self.pane_distance)).atan()
    }
    #[deprecated = "Use pane distance instead"]
    pub fn set_fov(&mut self, fov: f32) {
        //! Sets the camera field of view. Value should be between ...
        //! ## Parameter
        //! fov: new field of view
        // quick hack for now
        /* self.pane_width = fov;
        self.pane_height =
            fov * (self.get_resolution().height as f32 / self.get_resolution().width as f32); */
        self.pane_distance = fov;
    }
    pub fn set_pane_distance(&mut self, distance: f32) {
        //! Set the camera pane distance
        //! ## Parameter
        //! distance: New value for pane_distance
        if distance >= 0.0 {
            self.pane_distance = distance;
        }
    }
    pub fn get_pane_distance(&self) -> f32 {
        //! ## Returns
        //! Camera pane distance
        self.pane_distance
    }
    pub fn set_pane_width(&mut self, width: f32) {
        //! Set the camera pane width
        //! ## Parameter
        //! width: New value for pane_distance
        if width > 0.0 {
            self.pane_width = width;
        }
    }
    pub fn get_pane_width(&self) -> f32 {
        //! ## Returns
        //! Camera pane width
        self.pane_width
    }
    pub fn get_resolution(&self) -> &Resolution {
        //! ## Returns
        //! Camera resolution as Array of u32
        &self.resolution
    }
    pub fn set_resolution(&mut self, resolution: Resolution) {
        //! Sets the camera resolution
        //! ## Parameter
        //! 'resolution': New resolution as array of u32
        self.resolution = resolution
    }
    pub fn new(position: Vec3, rotation: Vec3) -> Self {
        //! Constructor for Camera
        //! ## Parameter
        //! 'position': Position of the new Camera as glam::Vec3
        //! 'rotation': Rotation of the new Camera as glam::Vec3
        //! # Returns
        //! A new camera with the given position and rotation
        let mut res = Camera::default();
        res.set_position(position);
        res.set_look_at(rotation);
        res
    }
    pub fn get_ray_samples(&self) -> u32 {
        self.ray_samples
    }
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
            position: Vec3::default(),
            resolution,
            ray_samples: 20,
            look_at: Vec3::new(0.0, 0.0, 1.0),
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
#[derive(Debug, Clone, Copy)]
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
