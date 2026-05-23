use glam::Vec3;

use crate::movement::Movement;
use crate::movement::Rotate;
use crate::util::Ray;

pub trait CameraInt {
    fn get_ray(&self, i: usize, j: usize) -> &Ray;
    // Forward projection: world point -> (screen_j, screen_i, depth). Returns
    // None if the point is at or behind the near plane (depth <= 0).
    fn project(&self, p_world: Vec3) -> Option<(f32, f32, f32)>;
    fn eye(&self) -> Vec3;
    fn forward(&self) -> Vec3;
}

pub trait Camera: CameraInt + Sync {}

// Apply the same Rotate the ray-grid constructor uses to a local basis vector
// so the rasterizer's world-axis basis is consistent with the ray grid.
fn rotated_dir(rot: &Rotate, mut v: Vec3) -> Vec3 {
    rot.update_direction(1.0, &mut v);
    v
}

pub struct OrthoCamera {
    rays: Vec<Vec<Ray>>,
    eye: Vec3,
    forward: Vec3,
    right_world: Vec3,
    up_world: Vec3,
    scale: f32,
    w: usize,
    h: usize,
}

impl OrthoCamera {
    pub fn new(d: Vec3, p: Vec3, scale: f32, w: usize, h: usize) -> Self {
        // Compute rotate axis and degree.
        let rot = Rotate::get(Vec3::new(-1., 0., 0.), d, Vec3::ZERO);

        // Pre-compute all rays
        let mut rays: Vec<Vec<Ray>> = vec![];
        for i in 0..h {
            rays.push(vec![]);
            let z = ((h as f32) / 2. - (i as f32)) * 2. / scale;
            for j in 0..w {
                let y = (-(w as f32) / 2. + (j as f32)) / scale;
                let mut p0 = Vec3::new(0., y, z);
                rot.update_point(1., &mut p0); // Rotate
                p0 += p; // Translate
                rays[i].push(Ray { p: p0, d: d });
            }
        }
        let right_world = rotated_dir(&rot, Vec3::new(0., 1., 0.));
        let up_world = rotated_dir(&rot, Vec3::new(0., 0., 1.));
        OrthoCamera {
            rays,
            eye: p,
            forward: d,
            right_world,
            up_world,
            scale,
            w,
            h,
        }
    }
}

impl CameraInt for OrthoCamera {
    fn get_ray(&self, i: usize, j: usize) -> &Ray {
        &self.rays[i][j]
    }

    fn project(&self, p_world: Vec3) -> Option<(f32, f32, f32)> {
        let v = p_world - self.eye;
        let z = v.dot(self.forward);
        if z <= 1e-3 {
            return None;
        }
        let y_eye = v.dot(self.right_world);
        let z_eye = v.dot(self.up_world);
        // Inverse of the ray-grid mapping: j = y_local * scale + w/2;
        //                                  i = h/2 - z_local * scale / 2.
        let j = y_eye * self.scale + (self.w as f32) / 2.0;
        let i = (self.h as f32) / 2.0 - z_eye * self.scale / 2.0;
        Some((j, i, z))
    }

    fn eye(&self) -> Vec3 {
        self.eye
    }

    fn forward(&self) -> Vec3 {
        self.forward
    }
}

unsafe impl Sync for OrthoCamera {}

impl Camera for OrthoCamera {}

pub struct PerspectiveCamera {
    rays: Vec<Vec<Ray>>,
    eye: Vec3,
    forward: Vec3,
    right_world: Vec3,
    up_world: Vec3,
    scale: f32,
    focal: f32,
    w: usize,
    h: usize,
}

impl PerspectiveCamera {
    pub fn new(d: Vec3, p: Vec3, scale: f32, f: f32, w: usize, h: usize) -> Self {
        let o = Vec3::new(f, 0., 0.);
        let rot = Rotate::get(Vec3::new(-1., 0., 0.), d, o);
        let mut rays: Vec<Vec<Ray>> = vec![];
        for i in 0..h {
            rays.push(vec![]);
            let z = ((h as f32) / 2. - (i as f32)) * 2. / scale;
            for j in 0..w {
                let y = (-(w as f32) / 2. + (j as f32)) / scale;
                let mut p0 = Vec3::new(0., y, z);
                rot.update_point(1., &mut p0); // Rotate
                p0 += p - o; // Translate
                rays[i].push(Ray {
                    p: p,
                    d: (p0 - p).normalize(),
                })
            }
        }
        let right_world = rotated_dir(&rot, Vec3::new(0., 1., 0.));
        let up_world = rotated_dir(&rot, Vec3::new(0., 0., 1.));
        PerspectiveCamera {
            rays,
            eye: p,
            forward: d,
            right_world,
            up_world,
            scale,
            focal: f,
            w,
            h,
        }
    }
}

impl CameraInt for PerspectiveCamera {
    fn get_ray(&self, i: usize, j: usize) -> &Ray {
        &self.rays[i][j]
    }

    fn project(&self, p_world: Vec3) -> Option<(f32, f32, f32)> {
        let v = p_world - self.eye;
        let z = v.dot(self.forward);
        if z <= 1e-3 {
            return None;
        }
        let y_eye = v.dot(self.right_world);
        let z_eye = v.dot(self.up_world);
        // Pinhole projection onto image plane at distance `focal` ahead of eye.
        let y_img = y_eye * self.focal / z;
        let z_img = z_eye * self.focal / z;
        let j = y_img * self.scale + (self.w as f32) / 2.0;
        let i = (self.h as f32) / 2.0 - z_img * self.scale / 2.0;
        Some((j, i, z))
    }

    fn eye(&self) -> Vec3 {
        self.eye
    }

    fn forward(&self) -> Vec3 {
        self.forward
    }
}

unsafe impl Sync for PerspectiveCamera {}

impl Camera for PerspectiveCamera {}
