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

// Brightness ramp, sparse to dense. Used by lum_to_char when --sharpen is off.
// Expanded from the original 13-char ramp for finer gradation.
pub const BRIGHTNESS_RAMP: &[char] = &[
    ' ', '.', '\'', '`', ',', ':', ';', '~', '-', '+', '=', '<', '>', '!', '*', '?', 'l',
    'i', '/', '\\', '|', '(', ')', 'o', 'x', 'X', '#', '%', '&', '$', '@', 'M',
];

pub fn lum_to_char(lum: f32) -> char {
    let n = BRIGHTNESS_RAMP.len();
    let i = (lum.clamp(0.0, 0.99999) * n as f32).floor() as usize;
    BRIGHTNESS_RAMP[i]
}

// Compute total luminance at a surface point, optionally checking shadow rays
// against every object. Matches the original get_color logic minus the
// brightness-ramp lookup so callers can either go to a char or feed a sub-cell
// lum buffer (for --sharpen).
pub fn get_lum(
    lights: &Vec<Box<dyn Light>>,
    objects: &Vec<Box<dyn Thing>>,
    p: Vec3,
    n: Vec3,
    out_d: Vec3,
    disable_shade: bool,
) -> f32 {
    let mut lum: f32 = 0.;
    for l in lights {
        // Check for blocking
        let ray = l.get_ray(p + 0.001 * n);
        let mut blocked: bool = false;
        if !disable_shade {
            for obj in objects {
                match obj.intersect(&ray) {
                    Some(_) => {
                        blocked = true;
                        break;
                    }
                    None => {}
                }
            }
        }
        if !blocked {
            lum += l.get_lum(p, n, out_d);
        }
    }
    lum
}

pub fn get_color(
    lights: &Vec<Box<dyn Light>>,
    objects: &Vec<Box<dyn Thing>>,
    p: Vec3,
    n: Vec3,
    out_d: Vec3,
    disable_shade: bool,
) -> char {
    lum_to_char(get_lum(lights, objects, p, n, out_d, disable_shade))
}
