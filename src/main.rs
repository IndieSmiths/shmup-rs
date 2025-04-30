
mod sdlsetup;
mod texturemap;
mod state;
mod gamestruct;
mod struct2d;

use std::time::Duration;

use sdlsetup::setup_and_get_structs;

use texturemap::get_texture_map;

use gamestruct::GameStruct;

use state::{State, get_state_map};
use state::loopholdertrait::LoopHolder;



pub fn main() -> Result<(), String> {

    let mut sdl_structs = setup_and_get_structs().unwrap();

    let texture_map = get_texture_map(&sdl_structs.texture_creator);

    let mut game_struct = GameStruct::new();

    let mut state_map = get_state_map(&texture_map, &mut game_struct);

    let State::GameState(state) = state_map.get_mut("game").unwrap();

    let quit = "quit".to_string();

    loop {

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));

        if let Err(text) = state.apploop(
                               &mut sdl_structs.event_pump,
                               &mut sdl_structs.canvas,
                               &mut game_struct,
                           ) {

            if text == quit {
                break;
            }

        }

    }

    Ok(())

}
