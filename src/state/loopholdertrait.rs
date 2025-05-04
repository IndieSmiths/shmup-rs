
use std::collections::HashMap;

use sdl2::render::Texture;
use sdl2::EventPump;
use sdl2::render::WindowCanvas;

use crate::gamestruct::GameStruct;


pub trait LoopHolder {

    fn get_input<'a>(
        &mut self,
        event_pump: &mut EventPump,
        game_struct: &mut GameStruct<'a>,
        texture_map: &'a HashMap<String, Texture<'a>>,
    ) -> Result<(), String>;

    fn update<'a>(
        &mut self,
        game_struct: &mut GameStruct<'a>,
        texture_map: &'a HashMap<String, Texture<'a>>,
    ) -> Result<(), String> {
        Ok(())
    }

    fn draw(
        &self,
        canvas: &mut WindowCanvas,
        game_struct: &GameStruct,
    )-> Result<(), String>;

    fn apploop<'a>(
        &mut self,
        event_pump: &mut EventPump,
        canvas: &mut WindowCanvas,
        game_struct: &mut GameStruct<'a>,
        texture_map: &'a HashMap<String, Texture<'a>>,
    ) -> Result<(), String> {

        self.get_input(event_pump, game_struct, texture_map)?;
        self.update(game_struct, texture_map)?;
        self.draw(canvas, game_struct)?;

        Ok(())

    }
}
