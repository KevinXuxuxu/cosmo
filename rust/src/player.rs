use std::thread;
use std::time::Duration;
use std::time::Instant;

use rayon::prelude::*;

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
    pub a: Vec<Vec<Color>>,
    t: f32,
    dt: f32,
    objects: Vec<Box<dyn Thing>>,
    camera: Box<dyn Camera>,
    lights: Vec<Box<dyn Light>>,
    disable_shade: bool,
    debug: bool,
}

impl Player {
    pub fn new(
        fr: i32,
        w: usize,
        h: usize,
        camera: Box<dyn Camera>,
        disable_shade: bool,
        debug: bool,
    ) -> Self {
        let a = vec![vec![' '; w]; h];
        let dt = if debug { 1.0 } else { 1.0 / (fr as f32) };
        Player {
            w,
            h,
            a,
            t: 0.,
            dt,
            camera,
            objects: vec![],
            lights: vec![],
            disable_shade,
            debug: debug,
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

        // Update lights
        for light in &mut self.lights {
            light.update(self.t, self.dt);
        }

        // Render
        self.a.par_iter_mut().enumerate().for_each(|(i, row)| {
            for j in 0..self.w {
                row[j] = ' ';
                let mut dist = f32::MAX;
                let ray = self.camera.get_ray(i, j);
                for obj in &self.objects {
                    match obj.intersect(ray) {
                        Some((p, n, c)) => {
                            let cur_dist = (p - ray.p).dot(ray.d);
                            if cur_dist > dist {
                                continue;
                            }
                            dist = cur_dist;
                            row[j] = if self.lights.len() > 0 {
                                get_color(
                                    &self.lights,
                                    &self.objects,
                                    p,
                                    n,
                                    ray.d,
                                    self.disable_shade,
                                )
                            } else {
                                c
                            };
                        }
                        None => {}
                    }
                }
            }
        });
    }

    pub fn run(&mut self, duration: f32) {
        let mut total_wait: f32 = 0.;
        let mut total_compute: f32 = 0.;
        loop {
            let start = Instant::now();
            self.update();
            if !self.debug {
                self.render();
            }
            let compute_t = start.elapsed().as_secs_f32();
            let wait_t: f32 = if self.dt >= compute_t {
                self.dt - compute_t
            } else {
                0.
            };
            if !self.debug {
                println!(
                    "{}compute_time: {:>8.5}ms wait_time: {:>8.5}ms",
                    CLEAR_LINE,
                    compute_t * 1000.,
                    wait_t * 1000.
                );
            }
            total_compute += compute_t;
            total_wait += wait_t;
            self.t += self.dt;
            if self.t > duration {
                break;
            }
            thread::sleep(Duration::from_secs_f32(wait_t));
        }
        let load = total_compute * 100. / (total_compute + total_wait);
        if !self.debug {
            println!(
                "total_compute: {}ms total_wait: {}ms load: {}%",
                total_compute * 1000.,
                total_wait * 1000.,
                load
            );
        }
    }
}
