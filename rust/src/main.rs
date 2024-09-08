use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use clap::Parser;
use rayon::ThreadPoolBuilder;
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    terminal::{disable_raw_mode, enable_raw_mode},
};

use crate::loader::parse_file;
use crate::player::Player;
use crate::control::ControlState;

pub mod aabb;
pub mod camera;
pub mod engine;
pub mod light;
pub mod loader;
pub mod movement;
pub mod player;
pub mod util;
pub mod control;

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

    #[arg(long, default_value_t = false)]
    aabb: bool,

    #[arg(long, default_value_t = false)]
    debug: bool,

    #[arg(long, default_value_t = false)]
    load_only: bool,

    #[arg(long, default_value_t = false)]
    disable_shade: bool,

    #[arg(long, default_value_t = 1)]
    n_threads: usize,
}

fn parse_size(v: &String) -> (usize, usize) {
    let pair: Vec<_> = v.split(',').collect();
    let w = pair[0].parse::<usize>().unwrap();
    let h = pair[1].parse::<usize>().unwrap();
    (w, h)
}

fn main() {
    let state = Arc::new(Mutex::new(ControlState::default()));
    let state_clone = Arc::clone(&state);
    enable_raw_mode();
    thread::spawn(move || {
        loop {
            // Poll for events with a short timeout
            if event::poll(Duration::from_millis(200)).unwrap() {
                if let Event::Key(KeyEvent {
                    code,
                    modifiers: KeyModifiers::NONE, // Only handle non-modified arrow keys
                    ..
                }) = event::read().unwrap()
                {
                    let mut states = state_clone.lock().unwrap();
                    match code {
                        KeyCode::Up => {
                            states.up = true;
                            states.down = false;
                            states.left = false;
                            states.right = false;
                        }
                        KeyCode::Down => {
                            states.up = false;
                            states.down = true;
                            states.left = false;
                            states.right = false;
                        }
                        KeyCode::Left => {
                            states.up = false;
                            states.down = false;
                            states.left = true;
                            states.right = false;
                        }
                        KeyCode::Right => {
                            states.up = false;
                            states.down = false;
                            states.left = false;
                            states.right = true;
                        }
                        _ => {}
                    }
                }
            } else {
                let mut states = state_clone.lock().unwrap();
                states.up = false;
                states.down = false;
                states.left = false;
                states.right = false;
            }
        }
    });

    let args = Args::parse();
    let (w, h) = parse_size(&args.size);
    ThreadPoolBuilder::new()
        .num_threads(args.n_threads)
        .build_global()
        .unwrap();

    let (objs, camera, lights) = parse_file(&args.filename, w, h, args.debug, args.aabb);
    // Somehow setting hight to odd number will cause fuzz edge
    let mut p = Player::new(args.fr, w, h, camera, args.disable_shade, state, args.debug);
    for obj in objs {
        p.add_object(obj);
    }
    for light in lights {
        p.add_light(light);
    }
    if !&args.load_only {
        p.run(args.duration);
    }

    disable_raw_mode();
}
