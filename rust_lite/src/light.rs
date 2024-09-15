use glam::Vec3;

use crate::util::Ray;

pub struct Light {
    pub d: Vec3,
    pub intensity: f32,
}

impl Light {
    pub fn get_luminance(&self, n: Vec3) -> char {
        let mut lum = 0.;
        if n.dot(self.d) < 0. {
            lum = self.intensity * (-n.dot(self.d));
        }
        let index: Vec<char> = vec![
            '.', ',', '-', '~', ':', ';', '=', '!', '*', '#', '$', '@', 'M',
        ];
        let i = (lum.min(0.99) * (index.len() as f32)).floor();
        index[i as usize]
    }
}
