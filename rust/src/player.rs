use std::thread;
use std::time::Duration;
use std::time::Instant;
use std::sync::{Arc, Mutex};
use std::io::{stdout, Write};

use rayon::prelude::*;
use glam::Vec3;
use crossterm::{ExecutableCommand, cursor};

use crate::camera::Camera;
use crate::engine::Thing;
use crate::light::{get_color, Light};
use crate::util::{Color, to_rad, Ray};
use crate::control::ControlState;
use crate::movement::Rotate;

const CURSOR_UP: &str = "\x1B[F";
const CLEAR_LINE: &str = "\x1B[K";

pub struct Player {
    // fr: i32,
    w: usize,
    h: usize,
    a: Vec<Vec<Color>>,
    t: f32,
    dt: f32,
    objects: Vec<Box<dyn Thing>>,
    camera: Box<dyn Camera>,
    lights: Vec<Box<dyn Light>>,
    disable_shade: bool,
    control_state: Arc<Mutex<ControlState>>,
    debug: bool,
}

impl Player {
    pub fn new(
        fr: i32,
        w: usize,
        h: usize,
        camera: Box<dyn Camera>,
        disable_shade: bool,
        control_state: Arc<Mutex<ControlState>>,
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
            control_state,
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
        let mut stdout = stdout();
        stdout.execute(cursor::MoveUp((self.h + 1).try_into().unwrap()));
        for l in &self.a {
            stdout.execute(cursor::MoveToColumn(0));
            let l_str: String = l.into_iter().collect();
            println!("{}", l_str);
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

        let state = self.control_state.lock().unwrap();
        if state.left {
            self.camera.update(self.dt, Box::new(Rotate {rad: -to_rad(60.), axis: Ray {p: Vec3::new(0., 0., 0.), d: Vec3::new(0., 0., 1.)}}));
        }
        if state.right {
            self.camera.update(self.dt, Box::new(Rotate {rad: to_rad(60.), axis: Ray {p: Vec3::new(0., 0., 0.), d: Vec3::new(0., 0., 1.)}}));
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
                            let cur_dist = (p - ray.p).length();
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
        let mut stdout = stdout();
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
                stdout.execute(cursor::MoveToColumn(0));
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
            stdout.execute(cursor::MoveToColumn(0));
            println!(
                "total_compute: {}ms total_wait: {}ms load: {}%",
                total_compute * 1000.,
                total_wait * 1000.,
                load
            );
        }
        stdout.execute(cursor::MoveToColumn(0));
    }
}
