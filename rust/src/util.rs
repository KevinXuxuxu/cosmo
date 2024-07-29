use glam::Vec3;

pub type Color = char;

#[derive(Default)]
pub struct Ray {
    pub p: Vec3,
    pub d: Vec3,
}
