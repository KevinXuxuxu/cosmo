use glam::f32::Vec3;

use crate::util::Ray;

pub trait Movement {
    fn update(&self, dt: f32, p: &mut Vec3);
}

#[derive(Default)]
pub struct Rotate {
    pub rad: f32,
    pub axis: Ray,
}

impl Rotate {
    pub fn get(d0: Vec3, d: Vec3, p: Vec3) -> Rotate {
        let axis_d = if d == d0 || d == -d0 {
            // This case d and d0 cannot form a plane, so no valid cross result.
            Vec3::new(0., 0., 1.)
        } else {
            d0.cross(d).normalize()
        };
        Rotate {
            rad: d0.dot(d).acos(),
            axis: Ray {
                p: p,
                d: axis_d,
            },
        }
    }
}

impl Movement for Rotate {
    fn update(&self, dt: f32, p: &mut Vec3) {
        let p_rel = *p - self.axis.p;
        let dr = self.rad * dt;
        let sin = dr.sin();
        let cos = dr.cos();
        let dot = self.axis.d.dot(p_rel);
        let cross = self.axis.d.cross(p_rel);
        let p_rot = p_rel * cos + cross * sin + self.axis.d * dot * (1. - cos);
        *p = p_rot + self.axis.p;
    }
}
