#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

use crate::loader::parse_scene;
use crate::player::Player;

pub mod aabb;
pub mod camera;
pub mod engine;
pub mod light;
pub mod loader;
pub mod movement;
pub mod player;
pub mod util;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub struct PlayerWASM {
    player: Player,
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
impl PlayerWASM {
    pub fn new(
        scene: Vec<String>,
        fr: i32,
        w: usize,
        h: usize,
        debug: bool,
        enable_aabb: bool,
        disable_shade: bool,
    ) -> Self {
        let (objs, camera, lights) = parse_scene(scene, w, h, debug, enable_aabb, None);
        let mut p = Player::new(fr, w, h, camera, disable_shade, debug);
        for obj in objs {
            p.add_object(obj);
        }
        for light in lights {
            p.add_light(light);
        }
        PlayerWASM { player: p }
    }

    pub fn get_a(&self) -> Vec<String> {
        let mut result: Vec<String> = vec![];
        for l in &self.player.a {
            let l_str: String = l.into_iter().collect();
            result.push(l_str);
        }
        result
    }

    pub fn update(&mut self) {
        self.player.update();
    }
}
