mod rect;
mod games;
mod scene_manager;
mod grid;
mod vec2;
mod vec3;

mod canvas;
mod input_state;

use crate::scene_manager::SceneManager;

fn main() {
    let mut scene_manager = SceneManager::new();
    scene_manager.run();
}
