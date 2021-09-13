mod games;
mod systems;
mod components;

mod rect;
mod scene_manager;
mod grid;
mod vec2;
mod vec3;

mod players;

mod canvas;
mod input_state;

use crate::scene_manager::SceneManager;

fn main() {
    let mut scene_manager = SceneManager::new();
    scene_manager.run();
}
