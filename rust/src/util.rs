use std::cmp::Ordering;
use std::path::{Path, PathBuf};

use glam::{Vec2, Vec3};

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

pub fn get_sphere_cord(d: &Vec3) -> Vec2 {
    // Assume d is normalized.
    Vec2::new(d.y.atan2(d.x), d.z.acos())
}

pub fn vec2_cmp(a: &Vec2, b: &Vec2) -> Ordering {
    if a.x < b.x || (a.x == b.x && a.y < b.y) {
        Ordering::Less
    } else {
        Ordering::Greater
    }
}

pub fn same_dir_file(filename: &str, base_file: &str) -> String {
    let base_path = Path::new(base_file);
    let dir = base_path.parent().unwrap_or_else(|| Path::new(""));
    let target_path: PathBuf = dir.join(filename);
    target_path.to_string_lossy().to_string()
}
