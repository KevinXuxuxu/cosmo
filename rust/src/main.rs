use std::thread;
use std::time::Duration;
use std::time::Instant;

use clap::Parser;
use glam::f32::Vec3;

use crate::engine::Color;
use crate::engine::Thing;
use crate::loader::parse_file;

pub mod engine;
pub mod loader;
pub mod movement;

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
    objects: Vec<Box<dyn Thing>>,
}

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

    pub fn add_object(&mut self, obj: Box<dyn Thing>) {
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
        // Update objects
        for obj in &mut self.objects {
            obj.update(self.t, self.dt);
        }

        // Render
        for i in 1..self.h {
            let z = ((self.h as f32) / 2. - (i as f32)) * 2.;
            for j in 0..self.w {
                self.a[i][j] = ' ';
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

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    filename: String,

    #[arg(long, default_value_t = 24)]
    fr: i32,

    #[arg(short, long)]
    size: String,

    #[arg(short, long)]
    duration: f32,
}

fn parse_size(v: &String) -> (usize, usize) {
    let pair: Vec<_> = v.split(',').collect();
    let w = pair[0].parse::<usize>().unwrap();
    let h = pair[1].parse::<usize>().unwrap();
    (w, h)
}

fn main() {
    let args = Args::parse();
    let (w, h) = parse_size(&args.size);

    // Somehow setting hight to odd number will cause fuzz edge
    let mut p = Player::new(args.fr, w, h);

    let objs = parse_file(&args.filename);
    for obj in objs {
        p.add_object(obj);
    }

    p.run(args.duration);
}
