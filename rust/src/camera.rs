use glam::Vec3;

use crate::movement::Movement;
use crate::movement::Rotate;
use crate::util::Ray;

pub trait Camera {
    fn get_ray(&self, i: usize, j: usize) -> &Ray;
}

pub struct OrthoCamera {
    // d: Vec3,
    // p: Vec3,
    // scale: f32,
    // w: usize,
    // h: usize,
    rays: Vec<Vec<Ray>>,
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
                rot.update(1., &mut p0); // Rotate
                p0 += p; // Translate
                rays[i].push(Ray { p: p0, d: d });
            }
        }
        OrthoCamera {
            // d,
            // p,
            // scale,
            // w,
            // h,
            rays,
        }
    }
}

impl Camera for OrthoCamera {
    fn get_ray(&self, i: usize, j: usize) -> &Ray {
        &self.rays[i][j]
    }
}

pub struct PerspectiveCamera {
    // d: Vec3,
    // p: Vec3,
    // scale: f32,
    // w: usize,
    // h: usize,
    // f: f32,
    rays: Vec<Vec<Ray>>,
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
                rot.update(1., &mut p0); // Rotate
                p0 += p - o; // Translate
                rays[i].push(Ray {
                    p: p,
                    d: (p0 - p).normalize(),
                })
            }
        }
        PerspectiveCamera {
            // d,
            // p,
            // scale,
            // w,
            // h,
            // f,
            rays,
        }
    }
}

impl Camera for PerspectiveCamera {
    fn get_ray(&self, i: usize, j: usize) -> &Ray {
        &self.rays[i][j]
    }
}
