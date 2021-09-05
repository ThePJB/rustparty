use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use std::collections::HashSet;


pub struct InputState {
    pub pressed_keys: HashSet<Keycode>,
    pub held_keys: HashSet<Keycode>,

    pub mouse_pos: (f32, f32), // range 0..a in x and 0..1 in y

    pub mouse_click_left: bool,
    pub mouse_click_right: bool,

    pub mouse_held_left: bool,
    pub mouse_held_right: bool,

    pub quit: bool,

    // also middle scroll etc
}

impl InputState {
    // returns the input state and if there was a quit event
    pub fn new(event_pump: &mut sdl2::EventPump, screen_x: u32, screen_y: u32) -> InputState {
        let a = screen_x as f32 / screen_y as f32;
        let mut input_state = InputState {
            quit: false,
            pressed_keys: HashSet::new(),
            held_keys: HashSet::new(),
            mouse_pos: (event_pump.mouse_state().x() as f32 * a / screen_x as f32,
                event_pump.mouse_state().y() as f32 / screen_y as f32),
            mouse_held_left: event_pump.mouse_state().left(),
            mouse_held_right: event_pump.mouse_state().right(),
            mouse_click_left: false,
            mouse_click_right: false,
        };

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit{..} => {input_state.quit = true;},
                Event::KeyDown{keycode: Some(keycode), ..} => {input_state.pressed_keys.insert(keycode);},
                Event::MouseButtonDown{mouse_btn: sdl2::mouse::MouseButton::Left, ..} => {input_state.mouse_click_left = true;},
                Event::MouseButtonDown{mouse_btn: sdl2::mouse::MouseButton::Right, ..} => {input_state.mouse_click_right = true;},
                _ => {},
            }
        }

        input_state.held_keys = event_pump.keyboard_state().pressed_scancodes().filter_map(Keycode::from_scancode).collect();
        return input_state;
    }
}