use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use std::collections::HashSet;
use std::collections::HashMap;


#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub enum Input {
    Up,
    Down,
    Left,
    Right,
    Action1,
    Action2,
}
pub type InputState = [PlayerInputState; 4];

pub struct PlayerInputState {
    pub new_inputs: Vec<Input>,
    pub held_inputs: HashSet<Input>,
}

pub fn translate_inputs(event_pump: &mut sdl2::EventPump, schema: &HashMap<Keycode, (usize, Input)>) -> (InputState, bool) {
    let mut player_inputs = [
        PlayerInputState{new_inputs: Vec::new(), held_inputs: HashSet::new()},
        PlayerInputState{new_inputs: Vec::new(), held_inputs: HashSet::new()},
        PlayerInputState{new_inputs: Vec::new(), held_inputs: HashSet::new()},
        PlayerInputState{new_inputs: Vec::new(), held_inputs: HashSet::new()}
    ];
            
    let mut quit = false;

    for event in event_pump.poll_iter() {
        match event {
            Event::Quit{..} => {quit = true;},
            Event::KeyDown{keycode: Some(keycode), ..} => {
                if let Some((player_number, input)) = schema.get(&keycode) {
                    player_inputs[*player_number].new_inputs.push(*input);
                }
            },
            _ => {},
        }
    }

    event_pump.keyboard_state().pressed_scancodes().filter_map(Keycode::from_scancode).for_each(|keycode| {
        if let Some((player_number, input)) = schema.get(&keycode) {
            player_inputs[*player_number].held_inputs.insert(*input);
        }
    });

    (player_inputs, quit)
}