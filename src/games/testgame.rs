use crate::scene_manager::Game;
use crate::scene_manager::Signal;
use crate::canvas::Canvas;
use crate::input_state::InputState;
use crate::input_state::Input;
use crate::vec2::Vec2;
use crate::vec3::Vec3;
use crate::rect::Rect;

#[derive(Clone, Copy, Debug)]
struct Entity {
    aabb: Rect,
    vel: Vec2,
}

#[derive(Debug)]
pub struct TestGame {
    players: [Entity; 4],

}
impl TestGame {
    pub fn new() -> TestGame {
        let game = TestGame{
            players: [
                Entity{aabb: Rect::new_centered(0.4, 0.4, 0.1, 0.1), vel: Vec2::zero()},
                Entity{aabb: Rect::new_centered(0.4, 0.6, 0.1, 0.1), vel: Vec2::zero()},
                Entity{aabb: Rect::new_centered(0.6, 0.4, 0.1, 0.1), vel: Vec2::zero()},
                Entity{aabb: Rect::new_centered(0.6, 0.6, 0.1, 0.1), vel: Vec2::zero()},
            ],
        };

        println!("game: {:?}", game);

        return game;
    }
}

impl Game for TestGame {
    fn update(&mut self, inputs: &InputState, dt: f64) -> Option<Signal> {
        // handle input
        for (player_number, state) in inputs.iter().enumerate() {
            self.players[player_number].vel.y = 
                if state.held_inputs.contains(&Input::Up) {
                    -1.0
                } else if state.held_inputs.contains(&Input::Down) {
                    1.0
                } else {
                    0.0
                };
            self.players[player_number].vel.x = 
                if state.held_inputs.contains(&Input::Left) {
                    -1.0
                } else if state.held_inputs.contains(&Input::Right) {
                    1.0
                } else {
                    0.0
                };
            self.players[player_number].vel = self.players[player_number].vel.normalize();
        }


        // update movement
        for player in self.players.iter_mut() {
            //println!("player aabb {:?} vel {:?} dt {:?}", player.aabb, player.vel, dt);
            player.aabb.x += player.vel.x * dt as f32;
            player.aabb.y += player.vel.y * dt as f32;
        }



        None
    }

    fn draw(&self, canvas: &mut Canvas) {
        canvas.clear(Vec3::new(1.0, 1.0, 1.0));

        for player in self.players {
            canvas.draw_rect(player.aabb, Vec3::new(1.0, 0.0, 0.0));
        }

        canvas.present();
    }
}