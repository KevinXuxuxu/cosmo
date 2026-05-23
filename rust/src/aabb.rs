use glam::Vec3;

use crate::util::Ray;

// Axis-aligned Bounding Box
#[derive(Default)]
pub struct AABB {
    x: Vec<f32>,
    y: Vec<f32>,
    z: Vec<f32>,
}

impl AABB {
    pub fn new() -> Self {
        AABB {
            x: vec![f32::MAX, f32::MIN],
            y: vec![f32::MAX, f32::MIN],
            z: vec![f32::MAX, f32::MIN],
        }
    }

    pub fn clear(&mut self) {
        self.x = vec![f32::MAX, f32::MIN];
        self.y = vec![f32::MAX, f32::MIN];
        self.z = vec![f32::MAX, f32::MIN];
    }

    pub fn update(&mut self, p: &Vec3) {
        self.x[0] = f32::min(p.x, self.x[0]);
        self.x[1] = f32::max(p.x, self.x[1]);
        self.y[0] = f32::min(p.y, self.y[0]);
        self.y[1] = f32::max(p.y, self.y[1]);
        self.z[0] = f32::min(p.z, self.z[0]);
        self.z[1] = f32::max(p.z, self.z[1]);
    }

    pub fn is_empty(&self) -> bool {
        self.x[0] > self.x[1] || self.y[0] > self.y[1] || self.z[0] > self.z[1]
    }

    pub fn min(&self) -> Vec3 {
        Vec3::new(self.x[0], self.y[0], self.z[0])
    }

    pub fn max(&self) -> Vec3 {
        Vec3::new(self.x[1], self.y[1], self.z[1])
    }

    pub fn centroid(&self) -> Vec3 {
        (self.min() + self.max()) * 0.5
    }

    // Expand to include `other`. No-op if `other` is empty so that callers
    // can fold a mix of populated and never-touched AABBs without inflating
    // the result.
    pub fn merge(&mut self, other: &AABB) {
        if other.is_empty() {
            return;
        }
        self.x[0] = f32::min(other.x[0], self.x[0]);
        self.x[1] = f32::max(other.x[1], self.x[1]);
        self.y[0] = f32::min(other.y[0], self.y[0]);
        self.y[1] = f32::max(other.y[1], self.y[1]);
        self.z[0] = f32::min(other.z[0], self.z[0]);
        self.z[1] = f32::max(other.z[1], self.z[1]);
    }

    // Standard slab method: clip the ray against three pairs of parallel
    // planes and keep the overlapping t-interval. The branchless form below
    // handles negative ray-direction components without an explicit swap,
    // and infinities arising from axis-aligned rays fall out correctly via
    // IEEE arithmetic.
    pub fn intersect(&self, ray: &Ray) -> bool {
        let mut tmin = f32::NEG_INFINITY;
        let mut tmax = f32::INFINITY;

        let axes = [
            (ray.p.x, ray.d.x, self.x[0], self.x[1]),
            (ray.p.y, ray.d.y, self.y[0], self.y[1]),
            (ray.p.z, ray.d.z, self.z[0], self.z[1]),
        ];

        for (o, d, bmin, bmax) in axes {
            let inv_d = 1.0 / d;
            let t1 = (bmin - o) * inv_d;
            let t2 = (bmax - o) * inv_d;
            tmin = tmin.max(t1.min(t2));
            tmax = tmax.min(t1.max(t2));
            if tmax < tmin {
                return false;
            }
        }
        tmax >= 0.0
    }
}
