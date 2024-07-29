use glam::f32::Vec3;

pub trait Movement {
    fn update(&self, dt: f32, p: &mut Vec3);
}

#[derive(Default)]
pub struct Rotate {
    rad: f32,
    axis_p: Vec3,
    axis_d: Vec3,
}

impl Rotate {
    pub fn new(degree: f32, axis_p: Vec3, axis_d: Vec3) -> Self {
        let rad = degree * (std::f32::consts::PI / 180.);
        let axis_d = axis_d.normalize();
        Rotate {
            rad,
            axis_p,
            axis_d
        }
    }
}

impl Movement for Rotate {
    fn update(&self, dt: f32, p: &mut Vec3) {
        let p_rel = *p - self.axis_p;
        let dr = self.rad * dt;
        let sin = dr.sin();
        let cos = dr.cos();
        let dot = self.axis_d.dot(p_rel);
        let cross = self.axis_d.cross(p_rel);
        let p_rot = p_rel * cos + cross * sin + self.axis_d * dot * (1. - cos);
        *p = p_rot + self.axis_p;
    }
}
