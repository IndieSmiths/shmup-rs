
use std::collections::{HashMap, HashSet};
use std::ops::Not;

use sdl2::EventPump;
use sdl2::event::Event;
use sdl2::keyboard::{Keycode, Scancode};
use sdl2::render::{WindowCanvas, Texture};
use sdl2::pixels::Color;

use crate::state::loopholdertrait::LoopHolder;
use crate::struct2d::{Player, Struct2D, EnemyA};
use crate::gamestruct::GameStruct;



pub struct Game<'a> {
    player: Player<'a>
}

impl<'a> Game<'a> {

    pub fn new(texture_map: &'a HashMap<String, Texture>, game_struct: &mut GameStruct<'a>) -> Result<Self, String> {

        let ea = EnemyA::new("center", (400, 400), &texture_map).unwrap();
        let var = Struct2D::VarEnemyA(ea);
        game_struct.actors.push(var);

        Ok(Self{ player: Player::new(&texture_map).unwrap()})
    }
}


impl LoopHolder for Game<'_> {

    fn get_input(&mut self, event_pump: &mut EventPump, game_struct: &mut GameStruct) -> Result<(), String> {

        for event in event_pump.poll_iter() {

            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    return Err(String::from("quit"));
                },
                _ => {}
            }

        }

        let pressed: HashSet<Scancode> = event_pump.keyboard_state().pressed_scancodes().collect();

        let (mut dx, mut dy) = (0, 0);

        if pressed.contains(&Scancode::W) {dy -= 1};
        if pressed.contains(&Scancode::A) {dx -= 1};
        if pressed.contains(&Scancode::S) {dy += 1};
        if pressed.contains(&Scancode::D) {dx += 1};

        if (dx == 0 && dy == 0).not() {
            dx *= 6;
            dy *= 6;
            self.player.rect.offset(dx, dy);
        }

        Ok(())

    }

    fn draw(&self, canvas: &mut WindowCanvas, game_struct: &GameStruct) -> Result<(), String> {

        canvas.set_draw_color(Color::RGB(100, 100, 100));
        canvas.clear();

        for struct2d in &game_struct.actors {

            match struct2d {

                Struct2D::VarEnemyA(ea) => 

                    canvas.copy(
                        &ea.texture,
                        None,
                        Some(ea.rect),
                    )?


            }

        }

        canvas.copy(
            &self.player.texture,
            None,
            Some(self.player.rect),
        )?;

        canvas.present();

        Ok(())

    }

}
