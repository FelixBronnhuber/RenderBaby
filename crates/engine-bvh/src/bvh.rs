//! Bounding Volume Hierarchy (BVH) construction.
use glam::Vec3;
use bytemuck::{Pod, Zeroable};

use crate::triangle::GPUTriangle;
use crate::aabb::AABB;

/// Maximum number of primitives stored in a leaf node.
///
/// Lower values typically improve traversal performance
/// at the cost of a deeper tree.
const MAX_LEAF_SIZE: usize = 128; //Maximum Triangles per Leaf, apparently lower is more common

/// A single node in the Bounding Volume Hierarchy.
///
/// The node layout is optimized for GPU usage and can represent
/// either an internal node or a leaf node.
#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable, Default)]
pub struct BVHNode {
    /// Minimum corner of the node's bounding box.
    pub aabb_min: Vec3,
    pub _pad0: u32,
    /// Maximum corner of the node's bounding box.
    pub aabb_max: Vec3,
    pub _pad1: u32,
    /// Indexes of child nodes
    pub left: u32,
    pub right: u32,
    /// Index of the first primitive stored in this node.
    pub first_primitive: u32,
    /// Number of primitives stored in this node.
    pub primitive_count: u32,
}

impl BVHNode {
    /// Creates a leaf node.
    ///
    /// Leaf nodes directly reference a range of primitives.
    pub fn leaf(
        aabb_min: Vec3,
        aabb_max: Vec3,
        first_primitive: u32,
        primitive_count: u32,
    ) -> Self {
        Self {
            aabb_min,
            aabb_max,
            first_primitive,
            primitive_count,
            ..Default::default()
        }
    }

    /// Creates an internal node.
    ///
    /// Internal nodes reference two child nodes and do not
    /// directly store primitives.
    pub fn internal(aabb_min: Vec3, aabb_max: Vec3, left: u32, right: u32) -> Self {
        Self {
            aabb_min,
            aabb_max,
            left,
            right,
            ..Default::default()
        }
    }
}

/// A Bounding Volume Hierarchy built over a set of triangles.
///
/// The BVH stores nodes in a flat array and keeps a separate
/// index buffer that references the original triangle list.
#[derive(Default)]
pub struct BVH {
    /// All BVH nodes in depth-first order.
    pub nodes: Vec<BVHNode>,
    /// Triangle indices referenced by leaf nodes.
    pub indices: Vec<u32>,
}

impl BVH {
    /// Builds a new BVH from a slice of triangles.
    ///
    /// The construction uses a median split along the longest axis
    /// of the node's bounding box.
    pub fn new(triangles: &[GPUTriangle]) -> Self {
        let mut indices: Vec<u32> = (0..triangles.len() as u32).collect();
        let mut nodes = Vec::new();

        build_node(triangles, &mut indices, &mut nodes, 0, triangles.len());

        Self { nodes, indices }
    }
}

/// Recursively builds a BVH node.
///
/// Returns the index of the newly created node.
fn build_node(
    triangles: &[GPUTriangle],
    indices: &mut [u32],
    nodes: &mut Vec<BVHNode>,
    first: usize,
    count: usize,
) -> u32 {
    let node_index = nodes.len() as u32;
    nodes.push(BVHNode::default());

    let mut aabb = AABB::empty();
    for i in first..first + count {
        let tri = &triangles[indices[i] as usize];
        aabb.expand(tri.v0); //Growing Box to include all Vertices, but keeping it minimal
        aabb.expand(tri.v1);
        aabb.expand(tri.v2);
    }

    if count <= MAX_LEAF_SIZE {
        //checks if the current Node is a Leaf
        //both left and right are 0 as they do not have any nodes underneath them, therefore referencing the root as default
        nodes[node_index as usize] = BVHNode::leaf(aabb.min, aabb.max, first as u32, count as u32);
        return node_index;
    }

    let axis = {
        //Splitting at longest axis
        let extent = aabb.max - aabb.min;
        if extent.x > extent.y && extent.x > extent.z {
            0
        } else if extent.y > extent.z {
            1
        } else {
            2
        }
    };

    let mid = first + count / 2; //partial sorting, better performance than actual sorting
    indices[first..first + count].select_nth_unstable_by(mid - first, |a, b| {
        let ca = triangle_centroid(&triangles[*a as usize])[axis];
        let cb = triangle_centroid(&triangles[*b as usize])[axis];
        ca.partial_cmp(&cb).unwrap()
    });

    let left = build_node(triangles, indices, nodes, first, mid - first);
    let right = build_node(triangles, indices, nodes, mid, first + count - mid);

    nodes[node_index as usize] = BVHNode::internal(aabb.min, aabb.max, left, right);

    node_index
}

/// Computes the centroid of a triangle.
fn triangle_centroid(tri: &GPUTriangle) -> Vec3 {
    (tri.v0 + tri.v1 + tri.v2) / 3.0
}
