use glam::f32::Vec3;

use crate::util::{get_norm_vec, Ray};

pub trait Movement {
    fn update_direction(&self, dt: f32, p: &mut Vec3);
    fn update_point(&self, dt: f32, p: &mut Vec3);
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
            get_norm_vec(d)
        } else {
            d0.cross(d).normalize()
        };
        Rotate {
            rad: d0.dot(d).acos(),
            axis: Ray { p: p, d: axis_d },
        }
    }
}

impl Movement for Rotate {
    fn update_direction(&self, dt: f32, p: &mut Vec3) {
        let dr = self.rad * dt;
        let sin = dr.sin();
        let cos = dr.cos();
        let dot = self.axis.d.dot(*p);
        let cross = self.axis.d.cross(*p);
        *p = (*p) * cos + cross * sin + self.axis.d * dot * (1. - cos);
    }
    fn update_point(&self, dt: f32, p: &mut Vec3) {
        *p = *p - self.axis.p;
        self.update_direction(dt, p);
        *p = *p + self.axis.p;
    }
}
