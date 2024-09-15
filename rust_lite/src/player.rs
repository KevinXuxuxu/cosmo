use std::thread;
use std::time::Duration;
use std::time::Instant;

use crate::camera::Camera;
use crate::triangle::Triangle;

const CURSOR_UP: &str = "\x1B[F";
const CLEAR_LINE: &str = "\x1B[K";

pub struct Player {
    w: usize,
    h: usize,
    a: Vec<Vec<char>>,
    dt: f32,
    triangles: Vec<Triangle>,
    camera: Camera,
}

impl Player {
    pub fn new(w: usize, h: usize, fr: usize, triangles: Vec<Triangle>, camera: Camera) -> Self {
        let a = vec![vec![' '; w]; h];
        let dt = 1.0 / (fr as f32);
        Player {
            w,
            h,
            a,
            dt,
            triangles,
            camera,
        }
    }

    pub fn render(&self) {
        println!("{}", CURSOR_UP.repeat(self.h)); // self.h + 1 if printing stats.
        for l in &self.a {
            let l_str: String = l.into_iter().collect();
            println!("{}{}", CLEAR_LINE, l_str);
        }
    }

    pub fn update(&mut self) {
        for i in 0..self.h {
            for j in 0..self.w {
                self.a[i][j] = ' ';
                for t in &self.triangles {
                    match t.intersect(&self.camera.rays[i][j]) {
                        Some(_) => {
                            self.a[i][j] = '.';
                            break;
                        }
                        None => {}
                    }
                }
            }
        }
    }

    pub fn play(&mut self, duration: f32) {
        let mut t = 0.;
        loop {
            let start = Instant::now();
            self.render();
            self.update();
            let compute_t = start.elapsed().as_secs_f32();
            let wait_t: f32 = if self.dt >= compute_t {
                self.dt - compute_t
            } else {
                0.
            };
            /*
            // Print stats
            println!(
                "{}compute_time: {} wait_time: {}",
                CLEAR_LINE, compute_t, wait_t
            );
            */
            t += self.dt;
            if t >= duration {
                break;
            };
            thread::sleep(Duration::from_secs_f32(wait_t));
        }
    }
}
