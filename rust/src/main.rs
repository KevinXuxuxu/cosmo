use std::thread;
use std::time::Duration;
use std::time::Instant;

use crate::engine::Visible;

pub mod engine;

const CURSOR_UP: &str = "\x1B[F";
const CLEAR_LINE: &str = "\x1B[K";

struct Player<'a> {
    // fr: i32,
    w: usize,
    h: usize,
    a: Vec<Vec<char>>,
    t: f32,
    dt: f32,
    dt_millis: u64,
    objects: Vec<&'a dyn Visible>
}

fn udiff(a: usize, b: usize) -> usize {
    return if a > b {a-b} else {b-a};
}

impl Player<'_> {
    fn new(fr: i32, w: usize, h: usize) -> Self {
        let empty_str = vec![' '; w];
        let a = vec![empty_str; h];
        let t = 0.0;
        let dt = 1.0 / (fr as f32);
        let dt_millis = (dt * 1000.0) as u64;
        let objects = vec![];
        Player { w, h, a, t, dt, dt_millis, objects}
    }

    pub fn render(&self) {
        println!("{}", CURSOR_UP.repeat(self.h + 1));
        for l in &self.a {
            let l_str: String = l.into_iter().collect();
            println!("{}{}", CLEAR_LINE, l_str);
        }
    }

    pub fn update(&mut self) {
        for i in 1..self.h {
            let i_f32 = (udiff(i, self.h/2) as f32).abs() * 2.0;
            for j in 0..self.w {
                let j_f32 = (udiff(j, self.w/2) as f32).abs();
                if (i_f32 * i_f32 + j_f32 * j_f32 - self.t * self.t).abs() < 30.0 {
                    self.a[i][j] = '#';
                } else {
                    self.a[i][j] = ' ';
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
            println!("{}compute_time: {} wait_time: {}", CLEAR_LINE, compute_t_millis, self.dt_millis - compute_t_millis);
            if self.t > duration {
                break;
            }
            thread::sleep(Duration::from_millis(self.dt_millis - compute_t_millis));
        }
    }
}

fn main() {
    let mut p = Player::new(24, 30, 30);
    p.run(20.);
}
