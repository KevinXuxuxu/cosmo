use glam::Vec3;

use crate::engine::Thing;
use crate::movement::Movement;
use crate::util::Ray;

pub trait LightInt {
    fn get_ray(&self, p: Vec3) -> Ray;
    fn get_lum(&self, p: Vec3, n: Vec3, out_d: Vec3) -> f32;
    fn update(&mut self, t: f32, dt: f32);
}

pub trait Light: LightInt + Sync {}

pub struct DirectionalLight {
    pub d: Vec3,
    pub l: f32,
    pub m: Option<Box<dyn Movement>>,
}

impl LightInt for DirectionalLight {
    fn get_ray(&self, p: Vec3) -> Ray {
        return Ray { p: p, d: -self.d };
    }

    fn get_lum(&self, _p: Vec3, n: Vec3, _out_d: Vec3) -> f32 {
        // TODO: Add surface properties (reflectiveness, etc.)
        if self.d.dot(n) > -1e-6 {
            return 0.;
        }
        return -self.d.dot(n) * self.l;
    }

    fn update(&mut self, _t: f32, dt: f32) {
        match &self.m {
            Some(mv) => {
                mv.update_direction(dt, &mut self.d);
            }
            None => {}
        }
    }
}

unsafe impl Sync for DirectionalLight {}

impl Light for DirectionalLight {}

pub struct PointLight {
    pub p: Vec3,
    pub l: f32,
    pub m: Option<Box<dyn Movement>>,
}

impl LightInt for PointLight {
    fn get_ray(&self, p: Vec3) -> Ray {
        return Ray {
            p: p,
            d: (self.p - p).normalize(),
        };
    }

    fn get_lum(&self, p: Vec3, n: Vec3, _out_d: Vec3) -> f32 {
        // TODO: Add surface properties (reflectiveness, etc.)
        let d = (p - self.p).normalize();
        if d.dot(n) > -1e-6 {
            return 0.;
        }
        let dis = (p - self.p).length();
        return -d.dot(n) * self.l / (dis * dis).max(1.);
    }

    fn update(&mut self, _t: f32, dt: f32) {
        match &self.m {
            Some(mv) => {
                mv.update_point(dt, &mut self.p);
            }
            None => {}
        }
    }
}

unsafe impl Sync for PointLight {}

impl Light for PointLight {}

pub fn get_color(
    lights: &Vec<Box<dyn Light>>,
    objects: &Vec<Box<dyn Thing>>,
    p: Vec3,
    n: Vec3,
    out_d: Vec3,
    disable_shade: bool,
) -> char {
    let mut lum: f32 = 0.; // TODO: Add ambient light
    for l in lights {
        // Check for blocking
        let ray = l.get_ray(p + 0.001 * n);
        let mut blocked: bool = false;
        if !disable_shade {
            for obj in objects {
                match obj.intersect(&ray) {
                    Some(_) => {
                        blocked = true;
                    }
                    None => {}
                }
            }
        }
        if !blocked {
            lum += l.get_lum(p, n, out_d);
        }
    }
    let brightness: Vec<char> = vec![
        '.', ',', '-', '~', ':', ';', '=', '!', '*', '#', '$', '@', 'M',
    ];
    let i = (lum.min(0.99999) * (brightness.len() as f32)).floor() as usize;
    brightness[i]
}
