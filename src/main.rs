
mod sdlsetup;
mod texturemap;
mod state;
mod gamestruct;
mod struct2d;
mod rectutils;

use std::time::Duration;

use crate::sdlsetup::setup_and_get_structs;

use crate::texturemap::get_texture_map;

use crate::gamestruct::GameStruct;

use crate::state::{State, get_state_map};
use crate::state::loopholdertrait::LoopHolder;



pub fn main() -> Result<(), String> {

    let mut sdl_structs = setup_and_get_structs().unwrap();

    let texture_map = get_texture_map(&sdl_structs.texture_creator);

    let mut game_struct = GameStruct::new(sdl_structs.canvas.logical_size());

    let mut state_map = get_state_map(&texture_map, &mut game_struct);

    let State::GameState(state) = state_map.get_mut("game").unwrap();

    let quit = "quit".to_string();

    loop {

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));

        if let Err(text) = state.apploop(
                               &mut sdl_structs.event_pump,
                               &mut sdl_structs.canvas,
                               &mut game_struct,
                               &texture_map,
                           ) {

            if text == quit {
                break;
            }

        }

    }

    Ok(())

}
