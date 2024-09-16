use glam::Vec3;

pub struct Ray {
    pub p: Vec3,
    pub d: Vec3,
}

pub fn rotate_z(rad: f32, dt: f32, p: &mut Vec3) {
    let a = rad * dt;
    *p = Vec3::new(
        p.x * a.cos() - p.y * a.sin(),
        p.x * a.sin() + p.y * a.cos(),
        p.z,
    );
}
