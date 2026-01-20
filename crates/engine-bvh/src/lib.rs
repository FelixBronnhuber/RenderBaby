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
