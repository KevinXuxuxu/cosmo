use glam::Vec3;

use crate::util::{rotate_z, Ray};

pub struct Triangle {
    a: Vec3,
    b: Vec3,
    c: Vec3,
    pub n: Vec3,
    rad: f32,
}

impl Triangle {
    pub fn new(a: Vec3, b: Vec3, c: Vec3, rad: f32) -> Self {
        let n = (b - a).cross(c - a).normalize();
        Triangle { a, b, c, n, rad }
    }

    pub fn intersect(&self, ray: &Ray) -> Option<Vec3> {
        let n_dot_d = self.n.dot(ray.d);
        if n_dot_d >= 0. {
            // Check for positive side
            return None;
        }
        // Solve for Q
        let t = self.n.dot(self.a - ray.p) / n_dot_d;
        let q = ray.p + t * ray.d;
        // Check if Q is in triangle
        if (self.b - self.a).cross(q - self.a).dot(self.n) < 0.
            || (self.c - self.b).cross(q - self.b).dot(self.n) < 0.
            || (self.a - self.c).cross(q - self.c).dot(self.n) < 0.
        {
            return None;
        }
        Some(q)
    }

    pub fn update(&mut self, dt: f32) {
        rotate_z(self.rad, dt, &mut self.a);
        rotate_z(self.rad, dt, &mut self.b);
        rotate_z(self.rad, dt, &mut self.c);
        self.n = (self.b - self.a).cross(self.c - self.a).normalize();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fn_intersect() {
        let t = Triangle::new(
            Vec3::new(0., 0., 0.),
            Vec3::new(0., 2., 0.),
            Vec3::new(-1., 0., 2.),
        );
        let rays: Vec<Ray> = vec![
            Ray {
                p: Vec3::new(0., 0.5, 0.5),
                d: Vec3::new(-1., 0., 0.),
            },
            Ray {
                p: Vec3::new(0., -1., 1.),
                d: Vec3::new(-1., 0., 0.),
            },
            Ray {
                p: Vec3::new(0., 1., -1.),
                d: Vec3::new(-1., 0., 0.),
            },
            Ray {
                p: Vec3::new(0., 1.5, 1.5),
                d: Vec3::new(-1., 0., 0.),
            },
        ];
        assert_eq!(t.intersect(&rays[0]).unwrap(), Vec3::new(-0.25, 0.5, 0.5));
        assert_eq!(t.intersect(&rays[1]), None);
        assert_eq!(t.intersect(&rays[2]), None);
        assert_eq!(t.intersect(&rays[3]), None);
    }
}
