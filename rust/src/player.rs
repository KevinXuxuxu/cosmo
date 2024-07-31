use std::thread;
use std::time::Duration;
use std::time::Instant;

use crate::camera::Camera;
use crate::engine::Thing;
use crate::light::{get_color, Light};
use crate::util::Color;

const CURSOR_UP: &str = "\x1B[F";
const CLEAR_LINE: &str = "\x1B[K";

pub struct Player {
    // fr: i32,
    w: usize,
    h: usize,
    a: Vec<Vec<Color>>,
    t: f32,
    dt: f32,
    dt_millis: u64,
    objects: Vec<Box<dyn Thing>>,
    camera: Box<dyn Camera>,
    lights: Vec<Box<dyn Light>>,
}

impl Player {
    pub fn new(fr: i32, w: usize, h: usize, camera: Box<dyn Camera>) -> Self {
        let empty_str = vec![' '; w];
        let a = vec![empty_str; h];
        let dt = 1.0 / (fr as f32);
        let dt_millis = (dt * 1000.0) as u64;
        Player {
            w,
            h,
            a,
            t: 0.,
            dt,
            dt_millis,
            camera,
            objects: vec![],
            lights: vec![],
        }
    }

    pub fn add_object(&mut self, obj: Box<dyn Thing>) {
        self.objects.push(obj);
    }

    pub fn add_light(&mut self, light: Box<dyn Light>) {
        self.lights.push(light);
    }

    pub fn render(&self) {
        println!("{}", CURSOR_UP.repeat(self.h + 1));
        for l in &self.a {
            let l_str: String = l.into_iter().collect();
            println!("{}{}", CLEAR_LINE, l_str);
        }
    }

    pub fn update(&mut self) {
        // Update objects
        for obj in &mut self.objects {
            obj.update(self.t, self.dt, None);
        }

        // Render
        for i in 0..self.h {
            for j in 0..self.w {
                self.a[i][j] = ' ';
                let ray = self.camera.get_ray(i, j);
                for obj in &self.objects {
                    match obj.intersect(ray) {
                        Some((p, n, c)) => {
                            self.a[i][j] = if self.lights.len() > 0 {
                                get_color(&self.lights, p, n, ray.d)
                            } else {
                                c
                            };
                            break;
                        }
                        None => {}
                    }
                }
            }
        }
    }

    pub fn run(&mut self, duration: f32) {
        let mut total_wait: u64 = 0;
        let mut total_compute: u64 = 0;
        loop {
            let start = Instant::now();
            self.update();
            self.render();
            let compute_t_millis = start.elapsed().as_millis() as u64;
            self.t += self.dt;
            let wait_t_millis = if self.dt_millis >= compute_t_millis {
                self.dt_millis - compute_t_millis
            } else {
                0
            };
            println!(
                "{}compute_time: {} wait_time: {}",
                CLEAR_LINE, compute_t_millis, wait_t_millis
            );
            total_compute += compute_t_millis;
            total_wait += wait_t_millis;
            if self.t > duration {
                break;
            }
            thread::sleep(Duration::from_millis(wait_t_millis));
        }
        let load = (total_compute as f32) * 100. / ((total_compute + total_wait) as f32);
        println!(
            "total_compute: {} total_wait: {} load: {}%",
            total_compute, total_wait, load
        );
    }
}
