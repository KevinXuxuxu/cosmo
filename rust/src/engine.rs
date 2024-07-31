use glam::f32::Vec3;

use crate::movement::Movement;
use crate::util::Color;
use crate::util::Ray;

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
                mv.update(dt, &mut self.a);
                mv.update(dt, &mut self.b);
                mv.update(dt, &mut self.c);
                self.process();
            }
            None => {}
        };
    }
}

impl Thing for Triangle {}

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
        return None
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