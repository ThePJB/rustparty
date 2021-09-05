// consumes minigame, so how do we maintain them
// i guess just choose an enum and have to dispatch it manually, thats fine i guess

use crate::canvas::Canvas;
use crate::input_state::InputState;

use std::time::SystemTime;
use std::time::Duration;

pub enum Signal {
    End,
}
pub trait Game {
    fn update(&mut self, inputs: &InputState, dt: f64) -> Option<Signal>;
    fn draw(&self, canvas: &mut Canvas);
}

pub struct SceneManager {
    current_game: Box<dyn Game>,
}



// maybe I provide games with floating point everything which means something other than a canvas
// maybe just like a ref mut to a 'canvas' with methods on it for drawing shit
// definitely want to adapt SDL to something better: proper mutability / immutability / per frame, and entirely floating point based

impl SceneManager {
    pub fn new() -> SceneManager {
        SceneManager {current_game: Box::new(crate::games::testgame::TestGame{})}
    }

    pub fn run(&mut self) {
        let xres = 1280;
        let yres = 720;
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem.window("rustparty", xres, yres)
            .position_centered()
            .build()
            .expect("failed making window");
    
        let mut sdl_canvas = window.into_canvas().build().expect("couldnt make canvas");
        let mut canvas = Canvas::new(&mut sdl_canvas);
        let mut event_pump = sdl_context.event_pump().unwrap();
        let mut dt = 1.0f64 / 60f64;

        'running: loop {
            let loop_start = SystemTime::now();

            let frame_input_state = InputState::new(&mut event_pump, xres, yres);

            if frame_input_state.quit { break 'running;}

            if let Some(signal) = self.current_game.update(&frame_input_state, dt) {
                match signal {
                    Signal::End => {break 'running},
                }
            }
            self.current_game.draw(&mut canvas);

            let loop_end = SystemTime::now();
            let delta = loop_end.duration_since(loop_start).unwrap().as_secs_f64();
            let frame_cap = 1.0 / 60.0;
            if delta < frame_cap {
                std::thread::sleep(Duration::from_secs_f64(frame_cap - delta));
                dt = frame_cap;
            } else {
                dt = delta;
            }
        }
    }
}
