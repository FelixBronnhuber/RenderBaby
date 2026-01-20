//! Bounding Volume Hierarchy / BVH
//! ## Features
//!
//! - **Triangle Representation**: [`GPUTriangle`] stores vertex positions and indices in a GPU-friendly layout.
//! - **Bounding Boxes**: [`AABB`] provides axis-aligned bounding boxes with utility methods like union and centroid computation.
//! - **Acceleration Structure**: [`BVH`] organizes triangles into a BVH for fast ray intersection queries.
//!
//! ## Core Modules
//!
//! - [`triangle`]: Defines the [`GPUTriangle`] type for GPU-compatible triangles.
//! - [`aabb`]: Defines [`AABB`] and related utilities for axis-aligned bounding boxes.
//! - [`bvh`]: Contains [`BVH`] and [`BVHNode`] for constructing acceleration structures.
//!
//! ## Usage Example
//!
//! ```rust, ignore
//! use engine-bvh::{triangle::GPUTriangle, aabb::AABB, bvh::BVH};
//! use glam::Vec3;
//! // Create a triangle
//! let tri = GPUTriangle {
//!     v0: Vec3::new(0.0, 0.0, 0.0),
//!     v1: Vec3::new(1.0, 0.0, 0.0),
//!     v2: Vec3::new(0.0, 1.0, 0.0),
//!     ..Default::default()
//! };
//!
//! // Build a BVH from a slice of triangles
//! let triangles = vec![tri];
//! let bvh = BVH::new(&triangles);
//!
//! // Compute an AABB enclosing a point
//! let mut aabb = AABB::empty();
//! aabb.expand(Vec3::new(1.0, 2.0, 3.0));
//! ```
//!
//! ## Architecture
//!
//! The system is designed around GPU-friendly layouts (`#[repr(C)]`) and flat
//! arrays to maximize performance for ray tracing or rendering applications.
//!
//! The BVH is built recursively using a median split along the longest axis
//! of each node's bounding box, producing leaf nodes with up to `MAX_LEAF_SIZE` triangles.
pub mod triangle;

pub mod aabb;

pub mod bvh;
