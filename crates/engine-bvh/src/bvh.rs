use glam::Vec3;
use bytemuck::{Pod, Zeroable};

use crate::triangle::GPUTriangle;
use crate::aabb::AABB;

const MAX_LEAF_SIZE: usize = 128; //Maximum Triangles per Leaf, apparently lower is more common

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable, Default)]
pub struct BVHNode {
    //Bounding Box of this Node
    pub aabb_min: Vec3,
    pub _pad0: u32,
    pub aabb_max: Vec3,
    pub _pad1: u32,
    //Index in Nodes Vector of BVH struct
    pub left: u32,
    pub right: u32,
    //Index of First Primitive in this Node and how many primitives there are in this node
    pub first_primitive: u32,
    pub primitive_count: u32,
    pub _pad2: [u32; 2],
}

pub struct BVH {
    pub nodes: Vec<BVHNode>,
    pub indices: Vec<u32>, // Only Triangles
}

impl BVH {
    pub fn new(triangles: &[GPUTriangle]) -> Self {
        let mut indices: Vec<u32> = (0..triangles.len() as u32).collect();
        let mut nodes = Vec::new();

        build_node(triangles, &mut indices, &mut nodes, 0, triangles.len());

        Self { nodes, indices }
    }
}

impl Default for BVH {
    fn default() -> Self {
        Self {
            nodes: Vec::new(),
            indices: Vec::new(),
        }
    }
}

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
        nodes[node_index as usize] = BVHNode {
            aabb_min: aabb.min,
            _pad0: 0,
            aabb_max: aabb.max,
            _pad1: 0,
            left: 0, //both are 0 as they do not have any nodes underneath them, therefore referencing the root as default
            right: 0,
            first_primitive: first as u32,
            primitive_count: count as u32,
            _pad2: [0, 0],
        };
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

    nodes[node_index as usize] = BVHNode {
        aabb_min: aabb.min,
        _pad0: 0,
        aabb_max: aabb.max,
        _pad1: 0,
        left,
        right,
        first_primitive: 0,
        primitive_count: 0,
        _pad2: [0, 0],
    };

    node_index
}

fn triangle_centroid(tri: &GPUTriangle) -> Vec3 {
    (tri.v0 + tri.v1 + tri.v2) / 3.0
}
