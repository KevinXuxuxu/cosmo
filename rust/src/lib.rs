#[cfg(target_arch = "wasm32")]
use js_sys::Uint8Array;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
use console_error_panic_hook;

use std::panic;
use std::collections::HashMap;

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
#[wasm_bindgen(start)]
pub fn main() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub struct PlayerWASM {
    player: Player,
}

#[cfg(target_arch = "wasm32")]
fn prepare_stl_data(
    stl_data_name: Vec<String>,
    stl_data: Vec<Uint8Array>,
) -> HashMap<String, Vec<u8>> {
    let mut map = HashMap::new();
    for i in 0..stl_data.len() {
        map.insert(stl_data_name[i].clone(), stl_data[i].to_vec());
    }
    map
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
impl PlayerWASM {
    pub fn new(
        scene: Vec<String>,
        fr: i32,
        w: usize,
        h: usize,
        enable_aabb: bool,
        disable_shade: bool,
        stl_data_name: Vec<String>,
        stl_data: Vec<Uint8Array>,
    ) -> Self {
        let (objs, camera, lights) = parse_scene(
            scene,
            w,
            h,
            false,
            enable_aabb,
            None,
            prepare_stl_data(stl_data_name, stl_data),
        );
        let mut p = Player::new(fr, w, h, camera, disable_shade, false);
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
