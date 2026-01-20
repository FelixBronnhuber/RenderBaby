//! Rendering engine type enumeration.
//!
//! This module defines the [`RenderEngine`] enum, which identifies the available
//! rendering backends in the RenderBaby system.

/// Available rendering engine types.
///
/// `RenderEngine` is used to select which rendering backend to use when creating
/// or switching engines. Each variant corresponds to a different rendering algorithm
/// with different characteristics.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, Default)]
pub enum RenderEngine {
    /// Classical ray tracing with direct lighting (not implemented).
    ///
    /// Features:
    /// - Fast rendering
    /// - Direct illumination
    /// - No global illumination
    /// - Good for previews
    Raytracer,

    /// Physically-based path tracing with global illumination (default).
    ///
    /// Features:
    /// - Accurate light transport
    /// - Global illumination
    /// - Indirect lighting
    /// - Slower but higher quality
    #[default]
    Pathtracer,
}
