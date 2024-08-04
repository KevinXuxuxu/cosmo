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

pub fn get_norm_vec(v: Vec3) -> Vec3 {
    if v == Vec3::ZERO {
        panic!("No normal vector for (0, 0, 0)");
    }
    if v.x == 0. {
        return v.cross(Vec3::new(1., 0., 0.)).normalize();
    }
    return v.cross(Vec3::new(0., 1., 0.)).normalize();
}
