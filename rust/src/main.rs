use glam::f32::Vec3;
use std::thread;
use std::time::Duration;
use std::time::Instant;

use crate::engine::Color;
use crate::engine::Visible;
use crate::loader::parse_file;

pub mod engine;
pub mod loader;

const CURSOR_UP: &str = "\x1B[F";
const CLEAR_LINE: &str = "\x1B[K";

#[derive(Default)]
struct Player {
    // fr: i32,
    w: usize,
    h: usize,
    a: Vec<Vec<Color>>,
    t: f32,
    dt: f32,
    dt_millis: u64,
    objects: Vec<Box<dyn Visible>>,
}

// fn udiff(a: usize, b: usize) -> usize {
//     return if a > b { a - b } else { b - a };
// }

impl Player {
    fn new(fr: i32, w: usize, h: usize) -> Self {
        let empty_str = vec![' '; w];
        let a = vec![empty_str; h];
        let dt = 1.0 / (fr as f32);
        let dt_millis = (dt * 1000.0) as u64;
        Player {
            w,
            h,
            a,
            dt,
            dt_millis,
            ..Default::default()
        }
    }

    pub fn add_object(&mut self, obj: Box<dyn Visible>) {
        self.objects.push(obj);
    }

    pub fn render(&self) {
        println!("{}", CURSOR_UP.repeat(self.h + 1));
        for l in &self.a {
            let l_str: String = l.into_iter().collect();
            println!("{}{}", CLEAR_LINE, l_str);
        }
    }

    pub fn update(&mut self) {
        // TODO: update logic
        for i in 1..self.h {
            let z = ((self.h as f32) / 2. - (i as f32)) * 2.;
            for j in 0..self.w {
                let y = -(self.w as f32) / 2. + (j as f32);
                for obj in &self.objects {
                    // TODO: add covering detect to support multiple objects
                    match obj.intersect(Vec3::new(0., y, z), Vec3::new(-1., 0., 0.)) {
                        Some(c) => {
                            self.a[i][j] = c;
                            break;
                        }
                        None => {}
                    }
                }
            }
        }
    }

    pub fn run(&mut self, duration: f32) {
        loop {
            let start = Instant::now();
            self.update();
            self.render();
            let compute_t_millis = start.elapsed().as_millis() as u64;
            self.t += self.dt;
            println!(
                "{}compute_time: {} wait_time: {}",
                CLEAR_LINE,
                compute_t_millis,
                self.dt_millis - compute_t_millis
            );
            if self.t > duration {
                break;
            }
            thread::sleep(Duration::from_millis(self.dt_millis - compute_t_millis));
        }
    }
}

fn main() {
    let objs = parse_file("scenes/square.cos");
    // Somehow setting hight to odd number will cause fuzz edge
    let mut p = Player::new(24, 30, 20);
    for obj in objs {
        p.add_object(obj);
    }
    p.run(5.);
}
