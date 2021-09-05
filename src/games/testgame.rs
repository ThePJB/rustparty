use crate::scene_manager::Game;
use crate::scene_manager::Signal;
use crate::canvas::Canvas;
use crate::input_state::InputState;
use crate::vec3::Vec3;


pub struct TestGame {

}

impl Game for TestGame {
    fn update(&mut self, inputs: &InputState, dt: f64) -> Option<Signal> {
        None
    }

    fn draw(&self, canvas: &mut Canvas) {
        canvas.clear(Vec3::new(1.0, 0.0, 0.0));
        canvas.present();
    }
}