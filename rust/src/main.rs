use std::thread;
use std::time::Duration;

const cursor_up: &str = "\x1B[F";
const clear_line: &str = "\x1B[K";

struct Player {
    fr: i32,
    w: usize,
    h: usize,
    a: Vec<Vec<char>>,
    t: f32,
}

fn udiff(a: usize, b: usize) -> usize {
    return if a > b {a-b} else {b-a};
}

impl Player {
    fn new(fr: i32, w: usize, h: usize) -> Self {
        let empty_str = vec![' '; w];
        let a = vec![empty_str; h];
        let t = 0.0;
        Player { fr, w, h, a, t }
    }

    pub fn render(&self) {
        println!("{}", cursor_up.repeat(self.h));
        for l in &self.a {
            let l_str: String = l.into_iter().collect();
            println!("{}{}", clear_line, l_str);
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
        let dt = 1.0 / (self.fr as f32);
        let dt_millis = (dt * 1000.0) as u64;
        thread::sleep(Duration::from_millis(dt_millis));
        self.t += dt;
    }
}

fn main() {
    let mut p = Player::new(24, 30, 30);
    loop {
        p.update();
        p.render();
        if p.t > 20.0 {
            break;
        }
    }
}
