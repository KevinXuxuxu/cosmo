use glam::f32::Vec3;

use crate::movement::{Movement, Rotate};
use crate::util::Color;
use crate::util::Ray;

const NEWTON_MAX_ITER: usize = 20;

pub trait Updatable {
    fn update(&mut self, t: f32, dt: f32, m: Option<&Box<dyn Movement>>);
}

pub trait Visible {
    fn intersect(&self, ray: &Ray) -> Option<(Vec3, Vec3, Color)>;
}

pub trait Thing: Updatable + Visible {}

#[derive(Default)]
pub struct Triangle {
    a: Vec3,
    b: Vec3,
    c: Vec3,
    color: Color,
    v0: Vec3,
    v1: Vec3,
    dot00: f32,
    dot01: f32,
    dot11: f32,
    inv_denom: f32,
    n: Vec3,
}

impl Triangle {
    pub fn new(a: Vec3, b: Vec3, c: Vec3, color: Color) -> Self {
        let mut t = Triangle {
            a,
            b,
            c,
            color,
            ..Default::default()
        };
        t.process();
        t
    }

    fn process(&mut self) {
        self.v0 = self.c - self.a;
        self.v1 = self.b - self.a;
        self.dot00 = self.v0.dot(self.v0);
        self.dot01 = self.v0.dot(self.v1);
        self.dot11 = self.v1.dot(self.v1);
        self.inv_denom = 1. / (self.dot00 * self.dot11 - self.dot01 * self.dot01);
        self.n = self.v1.cross(self.v0).normalize();
    }

    fn contains_point(&self, p: Vec3) -> bool {
        let v2 = p - self.a;
        let dot02 = self.v0.dot(v2);
        let dot12 = self.v1.dot(v2);
        let u = (self.dot11 * dot02 - self.dot01 * dot12) * self.inv_denom;
        let v = (self.dot00 * dot12 - self.dot01 * dot02) * self.inv_denom;
        u >= 0. && v >= 0. && u + v < 1.
    }
}

impl Visible for Triangle {
    fn intersect(&self, ray: &Ray) -> Option<(Vec3, Vec3, Color)> {
        let denom = self.n.dot(ray.d);
        if denom > -1e-6 {
            return None;
        }
        let t = self.n.dot(self.a - ray.p) / denom;
        let p = ray.p + t * ray.d;
        if self.contains_point(p) {
            Some((p, self.n, self.color))
        } else {
            None
        }
    }
}

impl Updatable for Triangle {
    fn update(&mut self, _t: f32, dt: f32, m: Option<&Box<dyn Movement>>) {
        match m {
            Some(mv) => {
                mv.update_point(dt, &mut self.a);
                mv.update_point(dt, &mut self.b);
                mv.update_point(dt, &mut self.c);
                self.process();
            }
            None => {}
        };
    }
}

impl Thing for Triangle {}

pub struct Sphere {
    pub o: Vec3,
    pub r: f32,
    pub color: Color,
}

impl Visible for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<(Vec3, Vec3, Color)> {
        let oc = ray.p - self.o;
        let a = ray.d.dot(ray.d);
        let b = 2.0 * oc.dot(ray.d);
        let c = oc.dot(oc) - self.r * self.r;
        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            None
        } else {
            let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
            let t2 = (-b + discriminant.sqrt()) / (2.0 * a);

            let t = if t1 >= 0.0 { t1 } else { t2 };

            if t >= 0.0 {
                let q = ray.p + t * ray.d;
                Some((q, (q - self.o).normalize(), self.color))
            } else {
                None
            }
        }
    }
}

impl Updatable for Sphere {
    fn update(&mut self, _t: f32, dt: f32, m: Option<&Box<dyn Movement>>) {
        match m {
            Some(mv) => {
                mv.update_point(dt, &mut self.o);
            }
            None => {}
        };
    }
}

impl Thing for Sphere {}

#[derive(Default)]
pub struct Torus {
    d: Vec3,
    p: Vec3,
    R: f32,
    r: f32,
    color: Color,
    debug: bool,
    rot: Rotate,
}

impl Torus {
    pub fn new(d: Vec3, p: Vec3, R: f32, r: f32, color: Color, debug: bool) -> Self {
        let mut t = Torus {
            d,
            p,
            R,
            r,
            color,
            debug,
            rot: Rotate {
                rad: 0.,
                axis: Ray {
                    p: Vec3::ZERO,
                    d: Vec3::ZERO,
                },
            },
        };
        t.process();
        t
    }

    fn process(&mut self) {
        self.rot = Rotate::get(self.d, Vec3::new(0., 0., 1.), Vec3::ZERO);
    }

    fn dt(&self, ray: &Ray, t: f32) -> (f32, f32) {
        let pt = ray.p + t * ray.d;
        let u = (pt.x.powi(2) + pt.y.powi(2)).sqrt();
        let f = (u - self.R).powi(2) + pt.z.powi(2) - self.r.powi(2);
        let du_dt = pt.x * ray.d.x / u + pt.y * ray.d.y / u;
        let df_dt = 2. * (u - self.R) * du_dt + 2. * pt.z * ray.d.z;
        (f, df_dt)
    }
}

impl Visible for Torus {
    fn intersect(&self, ray: &Ray) -> Option<(Vec3, Vec3, Color)> {
        // Transpose ray as if the torus is at standard position.
        let mut r_p = ray.p - self.p;
        let mut r_d = ray.d;
        self.rot.update_point(1., &mut r_p);
        self.rot.update_point(1., &mut r_d);
        let new_ray = Ray { p: r_p, d: r_d };
        // Use Newton's method to numerically solve intersection.
        let mut t: f32 = 0.01;
        let mut n: usize = 0;
        loop {
            let (f, df_dt) = self.dt(&new_ray, t);
            if df_dt.abs() <= f32::EPSILON {
                break;
            }
            let dt = f / df_dt;
            if dt.abs() < 1e-4 {
                break;
            }
            if n >= NEWTON_MAX_ITER {
                return None;
            }
            t -= dt;
            n += 1;
        }
        let mut p = new_ray.p + t * new_ray.d;
        let mut o = self.R * p.with_z(0.).normalize();
        // Transpose intersection and normal vec back to correct position.
        self.rot.update_point(-1., &mut p);
        self.rot.update_point(-1., &mut o);
        p += self.p;
        o += self.p;
        Some((p, (p - o).normalize(), self.color))
    }
}

impl Updatable for Torus {
    fn update(&mut self, _t: f32, dt: f32, m: Option<&Box<dyn Movement>>) {
        match m {
            Some(mv) => {
                mv.update_direction(dt, &mut self.d);
                mv.update_point(dt, &mut self.p);
                self.process();
            }
            None => {}
        };
    }
}

impl Thing for Torus {}

pub struct Object {
    pub children: Vec<Box<dyn Thing>>,
    pub m: Option<Box<dyn Movement>>,
}

impl Visible for Object {
    fn intersect(&self, ray: &Ray) -> Option<(Vec3, Vec3, Color)> {
        for child in &self.children {
            match child.intersect(ray) {
                Some(rtn) => return Some(rtn),
                _ => {}
            }
        }
        return None;
    }
}

impl Updatable for Object {
    fn update(&mut self, t: f32, dt: f32, _m: Option<&Box<dyn Movement>>) {
        match &self.m {
            Some(mv) => {
                for child in &mut self.children {
                    child.update(t, dt, Some(&mv));
                }
            }
            None => {}
        }
    }
}

impl Thing for Object {}
