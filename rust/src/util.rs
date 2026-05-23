use std::cmp::Ordering;
use std::path::{Path, PathBuf};

use glam::{Quat, Vec2, Vec3};

pub type Color = char;

#[derive(Default)]
pub struct Ray {
    pub p: Vec3,
    pub d: Vec3,
}

#[derive(Clone, Copy)]
pub struct Transform {
    pub rotation: Quat,
    pub translation: Vec3,
}

impl Transform {
    pub fn identity() -> Self {
        Transform {
            rotation: Quat::IDENTITY,
            translation: Vec3::ZERO,
        }
    }

    pub fn object_to_world_point(&self, p: Vec3) -> Vec3 {
        self.rotation * p + self.translation
    }

    pub fn object_to_world_dir(&self, d: Vec3) -> Vec3 {
        self.rotation * d
    }

    pub fn world_to_object_point(&self, p: Vec3) -> Vec3 {
        self.rotation.inverse() * (p - self.translation)
    }

    pub fn world_to_object_dir(&self, d: Vec3) -> Vec3 {
        self.rotation.inverse() * d
    }

    // Compose an incremental rotation about the line (axis_pivot, axis_dir).
    // Standard derivation: the incremental rotation R applied around `pivot` in
    // world space turns the existing transform (Q, t) into (R*Q, R*(t-pivot)+pivot).
    pub fn rotate_around(&mut self, axis_dir: Vec3, axis_pivot: Vec3, rad: f32) {
        let r_inc = Quat::from_axis_angle(axis_dir, rad);
        self.translation = r_inc * (self.translation - axis_pivot) + axis_pivot;
        self.rotation = r_inc * self.rotation;
    }
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
