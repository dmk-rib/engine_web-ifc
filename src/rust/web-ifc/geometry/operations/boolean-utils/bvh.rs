//! Rust port of `web-ifc/geometry/operations/boolean-utils/bvh.h`.

use std::cmp::Ordering;

use glam::DVec3;

use super::aabb::AABB;
use super::geometry::Geometry;

#[derive(Clone, Debug, Default)]
pub struct BVHNode {
    pub start: u32,
    pub end: u32,
    pub left: u32,
    pub right: u32,
    pub box_: AABB,
}

impl BVHNode {
    pub fn is_leaf(&self) -> bool {
        self.left == 0 && self.right == 0
    }

    pub fn overlaps(&self, other: &BVHNode) -> bool {
        self.box_.intersects(&other.box_)
    }
}

#[derive(Clone, Debug, Default)]
pub struct BVH<'a> {
    pub box_: AABB,
    pub boxes: Vec<AABB>,
    pub nodes: Vec<BVHNode>,
    pub ptr: Option<&'a Geometry>,
}

impl<'a> BVH<'a> {
    pub fn intersect<F>(&self, other: &BVH<'_>, mut callback: F)
    where
        F: FnMut(u32, u32),
    {
        let mut stack = Vec::new();
        stack.push((0u32, 0u32));

        while let Some((i1, i2)) = stack.pop() {
            let n1 = &self.nodes[i1 as usize];
            let n2 = &other.nodes[i2 as usize];

            if n1.overlaps(n2) {
                if n1.is_leaf() && n2.is_leaf() {
                    for i in n1.start..n1.end {
                        for j in n2.start..n2.end {
                            if self.boxes[i as usize].intersects(&other.boxes[j as usize]) {
                                callback(
                                    self.boxes[i as usize].index,
                                    other.boxes[j as usize].index,
                                );
                            }
                        }
                    }
                } else if n1.is_leaf() {
                    stack.push((i1, n2.left));
                    stack.push((i1, n2.right));
                } else if n2.is_leaf() {
                    stack.push((n1.left, i2));
                    stack.push((n1.right, i2));
                } else {
                    stack.push((n1.left, i2));
                    stack.push((n1.right, i2));
                }
            }
        }
    }

    pub fn intersect_ray<F>(&self, origin: DVec3, dir: DVec3, mut callback: F) -> bool
    where
        F: FnMut(u32) -> bool,
    {
        if self.nodes.is_empty() {
            return false;
        }

        let mut stack = Vec::new();
        stack.push(0u32);

        while let Some(node_index) = stack.pop() {
            let node = &self.nodes[node_index as usize];
            if node.box_.intersect(&origin, &dir) {
                if node.is_leaf() {
                    for i in node.start..node.end {
                        let box_ = &self.boxes[i as usize];
                        if box_.intersect(&origin, &dir) {
                            if callback(box_.index) {
                                return true;
                            }
                        }
                    }
                } else {
                    stack.push(node.left);
                    stack.push(node.right);
                }
            }
        }

        false
    }
}

fn make_bvh_recursive(
    boxes: &mut [AABB],
    nodes: &mut Vec<BVHNode>,
    start: usize,
    end: usize,
    axis: usize,
    depth: usize,
) -> u32 {
    let node_id = nodes.len() as u32;
    nodes.push(BVHNode::default());

    {
        let node = &mut nodes[node_id as usize];
        node.start = start as u32;
        node.end = end as u32;
        for i in start..end {
            node.box_.merge(&boxes[i]);
        }
    }

    if depth == 6 {
        return node_id;
    }

    let size = end - start;
    if size <= 12 {
        return node_id;
    }

    let middle = (end + start) / 2;
    boxes[start..end].sort_by(|a, b| {
        let av = a.center[axis];
        let bv = b.center[axis];
        bv.partial_cmp(&av).unwrap_or(Ordering::Equal)
    });

    let left = make_bvh_recursive(boxes, nodes, start, middle, (axis + 1) % 3, depth + 1);
    let right = make_bvh_recursive(boxes, nodes, middle, end, (axis + 1) % 3, depth + 1);

    let node = &mut nodes[node_id as usize];
    node.left = left;
    node.right = right;

    node_id
}

pub fn make_bvh(mesh: &Geometry) -> BVH<'_> {
    let mut bvh = BVH {
        ptr: Some(mesh),
        ..Default::default()
    };
    bvh.boxes = vec![AABB::default(); mesh.num_faces as usize];

    for i in 0..mesh.num_faces as usize {
        bvh.boxes[i] = mesh.get_face_box(i);
        bvh.box_.merge(&bvh.boxes[i]);
    }

    let _ = make_bvh_recursive(&mut bvh.boxes, &mut bvh.nodes, 0, bvh.boxes.len(), 0, 0);
    bvh
}
