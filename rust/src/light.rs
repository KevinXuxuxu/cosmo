use glam::Vec3;

pub trait Light {
    fn get_lum(&self, p: Vec3, n: Vec3, out_d: Vec3) -> f32;
}

pub struct DirectionalLight {
    pub d: Vec3,
    pub l: f32,
}

impl Light for DirectionalLight {
    fn get_lum(&self, _p: Vec3, n: Vec3, _out_d: Vec3) -> f32 {
        // TODO: Add surface properties (reflectiveness, etc.)
        if self.d.dot(n) > -1e-6 {
            return 0.;
        }
        return -self.d.dot(n) * self.l;
    }
}

pub struct PointLight {
    pub p: Vec3,
    pub l: f32,
}

impl Light for PointLight {
    fn get_lum(&self, p: Vec3, n: Vec3, _out_d: Vec3) -> f32 {
        // TODO: Add surface properties (reflectiveness, etc.)
        let d = (p - self.p).normalize();
        if d.dot(n) > -1e-6 {
            return 0.;
        }
        let dis = (p - self.p).length();
        return -d.dot(n) * self.l / (dis * dis).max(1.);
    }
}

pub fn get_color(lights: &Vec<Box<dyn Light>>, p: Vec3, n: Vec3, out_d: Vec3) -> char {
    let mut lum: f32 = 0.; // TODO: Add ambient light
    for l in lights {
        lum += l.get_lum(p, n, out_d);
    }
    let brightness: Vec<char> = vec![
        '.', ',', ':', ';', '!', '~', '-', '+', '=', '*', '#', '%', '@', 'M',
    ];
    let i = (lum.min(0.99999) * (brightness.len() as f32)).floor() as usize;
    brightness[i]
}
