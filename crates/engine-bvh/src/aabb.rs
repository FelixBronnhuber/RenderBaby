//! Axis-Aligned Bounding Box (AABB) utilities.
use glam::Vec3;

/// An axis-aligned bounding box defined by minimum and maximum corners.
///
/// This structure is commonly used for spatial partitioning,
/// collision detection, and acceleration structures such as BVHs.
#[derive(Clone, Copy, Debug)]
pub struct AABB {
    /// Minimum corner of the bounding box.
    pub min: Vec3,
    /// Maximum corner of the bounding box.
    pub max: Vec3,
}

impl AABB {
    /// Creates an empty bounding box.
    ///
    /// The resulting AABB has its minimum set to positive infinity
    /// and its maximum set to negative infinity. This makes it useful
    /// as a starting point that can be expanded incrementally.
    pub fn empty() -> Self {
        Self {
            min: Vec3::splat(f32::INFINITY),
            max: Vec3::splat(f32::NEG_INFINITY),
        }
    }

    /// Expands the bounding box to include the given point.
    ///
    /// If the point lies outside the current bounds, the AABB
    /// will grow just enough to contain it.
    pub fn expand(&mut self, p: Vec3) {
        self.min = self.min.min(p);
        self.max = self.max.max(p);
    }

    /// Computes the union of this AABB with another one.
    ///
    /// The returned bounding box encloses both input boxes.
    pub fn union(&self, other: &AABB) -> AABB {
        AABB {
            min: self.min.min(other.min),
            max: self.max.max(other.max),
        }
    }

    /// Returns the centroid (center point) of the bounding box.
    pub fn centroid(&self) -> Vec3 {
        (self.min + self.max) * 0.5
    }
}
