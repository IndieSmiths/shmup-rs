
use sdl2::EventPump;
use sdl2::render::{WindowCanvas};

use crate::gamestruct::GameStruct;


pub trait LoopHolder {

    fn get_input(
        &mut self,
        event_pump: &mut EventPump,
        game_struct: &mut GameStruct,
    ) -> Result<(), String>;

    fn update(
        &mut self,
        game_struct: &mut GameStruct,
    ) -> Result<(), String> {
        Ok(())
    }

    fn draw(
        &self,
        canvas: &mut WindowCanvas,
        game_struct: &GameStruct,
    )-> Result<(), String>;

    fn apploop(
        &mut self,
        event_pump: &mut EventPump,
        canvas: &mut WindowCanvas,
        game_struct: &mut GameStruct,
    ) -> Result<(), String> {

        self.get_input(event_pump, game_struct)?;
        self.update(game_struct)?;
        self.draw(canvas, game_struct)?;

        Ok(())

    }
}
