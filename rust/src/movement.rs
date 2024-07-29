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
