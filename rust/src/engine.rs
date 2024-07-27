use glam::f32::Vec3;

pub type Color = char;

pub trait Updatable {
    fn update(&mut self, t: f32);
}

pub trait Visible {
    fn intersect(&self, p0: Vec3, d: Vec3) -> Option<Color>;
}

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
    fn intersect(&self, p0: Vec3, d: Vec3) -> Option<Color> {
        let denom = self.n.dot(d);
        if denom > -1e-6 {
            return None;
        }
        let t = self.n.dot(self.a - p0) / denom;
        let p = p0 + t * d;
        if self.contains_point(p) {
            Some(self.color)
        } else {
            None
        }
    }
}
