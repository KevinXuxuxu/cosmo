use glam::Vec3;

use crate::util::Ray;

pub struct Camera {
    pub rays: Vec<Vec<Ray>>,
}

impl Camera {
    pub fn new(w: usize, h: usize, scale: f32) -> Self {
        let mut rays: Vec<Vec<Ray>> = vec![];
        for i in 0..h {
            rays.push(vec![]);
            let z = ((h as f32) / 2. - (i as f32)) * scale * 2.;
            for j in 0..w {
                let y = (-(w as f32) / 2. + (j as f32)) * scale;
                rays[i].push(Ray {
                    p: Vec3::new(0., y, z),
                    d: Vec3::new(-1., 0., 0.),
                });
            }
        }
        Camera { rays }
    }
}
