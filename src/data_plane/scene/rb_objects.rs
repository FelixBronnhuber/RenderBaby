use std::ops::{Deref, DerefMut};

use scene_objects::sphere::Sphere;

/// Deref pattern to extend scene objects with same behaviour
pub struct RbSphere(Sphere);

impl Deref for RbSphere {
    type Target = Sphere;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for RbSphere {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
// alternatively: use static fn in adapter
