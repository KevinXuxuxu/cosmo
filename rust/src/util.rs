use glam::Vec3;

pub type Color = char;

#[derive(Default)]
pub struct Ray {
    pub p: Vec3,
    pub d: Vec3,
}

pub fn to_rad(degree: f32) -> f32 {
    degree * (std::f32::consts::PI / 180.)
}
