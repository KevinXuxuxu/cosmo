use std::cmp::Ordering;

use glam::Vec3;

use crate::aabb::AABB;
use crate::engine::Thing;
use crate::util::{Color, Ray};

const LEAF_MAX: usize = 4;

pub enum Bvh {
    Internal {
        aabb: AABB,
        left: Box<Bvh>,
        right: Box<Bvh>,
    },
    Leaf {
        aabb: AABB,
        indices: Vec<usize>,
    },
}

impl Bvh {
    pub fn build(children: &[Box<dyn Thing>]) -> Self {
        if children.is_empty() {
            return Bvh::Leaf {
                aabb: AABB::new(),
                indices: vec![],
            };
        }
        let child_aabbs: Vec<AABB> = children
            .iter()
            .map(|c| {
                let mut a = AABB::new();
                c.update_aabb(&mut a);
                a
            })
            .collect();
        let indices: Vec<usize> = (0..children.len()).collect();
        build_recursive(indices, &child_aabbs)
    }

    pub fn intersect(
        &self,
        ray: &Ray,
        children: &[Box<dyn Thing>],
    ) -> Option<(Vec3, Vec3, Color)> {
        intersect_inner(self, ray, children).map(|(p, n, c, _)| (p, n, c))
    }
}

fn build_recursive(mut indices: Vec<usize>, child_aabbs: &[AABB]) -> Bvh {
    let aabb = union_of(&indices, child_aabbs);

    if indices.len() <= LEAF_MAX {
        return Bvh::Leaf { aabb, indices };
    }

    // Pick split axis as the longest extent of the centroid bounds.
    let mut cmin = Vec3::splat(f32::MAX);
    let mut cmax = Vec3::splat(f32::MIN);
    for &i in &indices {
        if child_aabbs[i].is_empty() {
            continue;
        }
        let c = child_aabbs[i].centroid();
        cmin = cmin.min(c);
        cmax = cmax.max(c);
    }
    let extent = cmax - cmin;
    let axis: usize = if extent.x >= extent.y && extent.x >= extent.z {
        0
    } else if extent.y >= extent.z {
        1
    } else {
        2
    };

    indices.sort_by(|&a, &b| {
        let ca = axis_value(&child_aabbs[a].centroid(), axis);
        let cb = axis_value(&child_aabbs[b].centroid(), axis);
        ca.partial_cmp(&cb).unwrap_or(Ordering::Equal)
    });

    let mid = indices.len() / 2;
    let right_indices = indices.split_off(mid);

    Bvh::Internal {
        aabb,
        left: Box::new(build_recursive(indices, child_aabbs)),
        right: Box::new(build_recursive(right_indices, child_aabbs)),
    }
}

fn union_of(indices: &[usize], child_aabbs: &[AABB]) -> AABB {
    let mut a = AABB::new();
    for &i in indices {
        a.merge(&child_aabbs[i]);
    }
    a
}

fn axis_value(v: &Vec3, axis: usize) -> f32 {
    match axis {
        0 => v.x,
        1 => v.y,
        _ => v.z,
    }
}

// Returns (point, normal, color, t) where t is the ray parameter at the hit.
// Keeping t around lets the traversal pick the nearest child without
// recomputing it, and would let callers compare across multiple BVHs later.
fn intersect_inner(
    node: &Bvh,
    ray: &Ray,
    children: &[Box<dyn Thing>],
) -> Option<(Vec3, Vec3, Color, f32)> {
    match node {
        Bvh::Internal { aabb, left, right } => {
            if !aabb.intersect(ray) {
                return None;
            }
            let l = intersect_inner(left, ray, children);
            let r = intersect_inner(right, ray, children);
            match (l, r) {
                (Some(a), Some(b)) => Some(if a.3 <= b.3 { a } else { b }),
                (Some(a), None) => Some(a),
                (None, Some(b)) => Some(b),
                (None, None) => None,
            }
        }
        Bvh::Leaf { aabb, indices } => {
            if indices.is_empty() || !aabb.intersect(ray) {
                return None;
            }
            let mut best: Option<(Vec3, Vec3, Color, f32)> = None;
            for &i in indices {
                if let Some((p, n, c)) = children[i].intersect(ray) {
                    let t = (p - ray.p).dot(ray.d);
                    match best {
                        Some((_, _, _, bt)) if bt <= t => {}
                        _ => best = Some((p, n, c, t)),
                    }
                }
            }
            best
        }
    }
}
