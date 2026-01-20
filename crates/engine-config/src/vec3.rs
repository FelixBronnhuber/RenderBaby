//! 3D vector implementation for positions, directions, and colors.
//!
//! This module provides a GPU-compatible 3D vector type used throughout the rendering system
//! for representing positions, directions, colors, and other 3-component data.

use bytemuck::{Pod, Zeroable};

/// A 3D vector with f32 components.
///
/// `Vec3` is a GPU-compatible wrapper around `[f32; 3]` that provides utility methods
/// for common vector operations and predefined constants for colors and common vectors.
///
/// # Memory Layout
///
/// The struct uses `#[repr(C)]` and derives `Pod` and `Zeroable` to ensure it can be
/// safely transmitted to GPU shaders without conversion.
///
/// # Usage
///
/// `Vec3` is used for:
/// - 3D positions (x, y, z coordinates)
/// - Direction vectors
/// - RGB color values (components should be in range 0.0-1.0)
/// - Normal vectors
#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable, Debug, PartialEq)]
pub struct Vec3(pub [f32; 3]);

impl Vec3 {
    /// Zero vector (0, 0, 0).
    pub const ZERO: Vec3 = Vec3([0.0, 0.0, 0.0]);
    /// Unit vector (1, 1, 1).
    pub const ONE: Vec3 = Vec3([1.0, 1.0, 1.0]);

    /// Pure red color (1, 0, 0).
    pub const COLOR_RED: Vec3 = Vec3([1.0, 0.0, 0.0]);
    /// Pure green color (0, 1, 0).
    pub const COLOR_GREEN: Vec3 = Vec3([0.0, 1.0, 0.0]);
    /// Pure blue color (0, 0, 1).
    pub const COLOR_BLUE: Vec3 = Vec3([0.0, 0.0, 1.0]);
    /// Yellow color (1, 1, 0).
    pub const COLOR_YELLOW: Vec3 = Vec3([1.0, 1.0, 0.0]);
    /// Cyan color (0, 1, 1).
    pub const COLOR_CYAN: Vec3 = Vec3([0.0, 1.0, 1.0]);
    /// Magenta color (1, 0, 1).
    pub const COLOR_MAGENTA: Vec3 = Vec3([1.0, 0.0, 1.0]);
    /// White color (1, 1, 1).
    pub const COLOR_WHITE: Vec3 = Vec3([1.0, 1.0, 1.0]);
    /// Black color (0, 0, 0).
    pub const COLOR_BLACK: Vec3 = Vec3([0.0, 0.0, 0.0]);

    /// Returns the x component.
    #[inline]
    pub fn x(&self) -> f32 {
        self.0[0]
    }

    /// Returns the y component.
    #[inline]
    pub fn y(&self) -> f32 {
        self.0[1]
    }

    /// Returns the z component.
    #[inline]
    pub fn z(&self) -> f32 {
        self.0[2]
    }

    /// Checks if all components are in the valid color range [0.0, 1.0].
    ///
    /// This is useful for validating RGB color values before they are used in rendering.
    ///
    /// # Returns
    ///
    /// `true` if all components are in [0.0, 1.0], `false` otherwise.
    pub fn is_valid_color(&self) -> bool {
        self.0.iter().all(|&c| (0.0..=1.0).contains(&c))
    }

    /// Creates a new `Vec3` from individual components.
    ///
    /// # Arguments
    ///
    /// * `x` - The x component
    /// * `y` - The y component
    /// * `z` - The z component
    ///
    /// # Returns
    ///
    /// A new `Vec3` with the specified components.
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vec3([x, y, z])
    }

    /// Scales the vector by a scalar value.
    ///
    /// Multiplies each component by `lambda`.
    ///
    /// # Arguments
    ///
    /// * `lambda` - The scalar multiplier
    ///
    /// # Returns
    ///
    /// A new `Vec3` with all components scaled by `lambda`.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// let v = Vec3::new(1.0, 2.0, 3.0);
    /// let scaled = v.scale(2.0);
    /// assert_eq!(scaled, Vec3::new(2.0, 4.0, 6.0));
    /// ```
    pub fn scale(&self, lambda: f32) -> Vec3 {
        Vec3([self.0[0] * lambda, self.0[1] * lambda, self.0[2] * lambda])
    }
}
