
use std::collections::HashMap;
use std::ops::Not;
use std::iter::Extend;

use sdl2::EventPump;
use sdl2::event::Event;
use sdl2::keyboard::{Keycode, Scancode};
use sdl2::render::{WindowCanvas, Texture};
use sdl2::pixels::Color;

use crate::state::loopholdertrait::LoopHolder;
use crate::struct2d::player::Player;
use crate::struct2d::actor::{Actors, EnemyA};
use crate::struct2d::projectile::{Projectiles, Shot000};
use crate::gamestruct::GameStruct;



pub struct Game<'a> {
    player: Player<'a>
}

impl<'a> Game<'a> {

    pub fn new(texture_map: &'a HashMap<String, Texture>, game_struct: &mut GameStruct<'a>) -> Result<Self, String> {

        let ea = EnemyA::new("center", (400, 400), &texture_map).unwrap();
        let var = Actors::VarEnemyA(ea);
        game_struct.actors.push(var);

        Ok(Self{ player: Player::new(&texture_map).unwrap()})
    }
}


impl LoopHolder for Game<'_> {

    fn get_input<'a>(
        &mut self,
        event_pump: &mut EventPump,
        game_struct: &mut GameStruct<'a>,
        texture_map: &'a HashMap<String, Texture<'a>>,
    ) -> Result<(), String> {

        // events

        for event in event_pump.poll_iter() {

            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    return Err(String::from("quit"));
                },
                _ => {}
            }

        }

        // --- button state ---

        let pressed = &mut game_struct.pressed_keys;

        pressed.extend(event_pump.keyboard_state().pressed_scancodes());

        let (w, a, s, d, j) = (
            pressed.contains(&Scancode::W),
            pressed.contains(&Scancode::A),
            pressed.contains(&Scancode::S),
            pressed.contains(&Scancode::D),
            pressed.contains(&Scancode::J),
        );

        pressed.clear();

        // moving

        let (mut dx, mut dy) = (0, 0);

        if w {dy -= 1};
        if a {dx -= 1};
        if s {dy += 1};
        if d {dx += 1};

        if (dx == 0 && dy == 0).not() {
            dx *= 6;
            dy *= 6;
            self.player.rect.offset(dx, dy);
        }

        // shooting

        if j {

            let midbottom = (
                self.player.rect.center().x(),
                self.player.rect.top(),
            );

            game_struct.request_shoot(midbottom ,&texture_map);
        }

        Ok(())

    }

    fn update(
        &mut self,
        game_struct: &mut GameStruct,
    ) -> Result<(), String> {
        
        game_struct.update();

        // TODO  probably create this vector in GameStruct instead,
        // for continuous reuse;

        let mut indices_to_remove: Vec<usize> = Vec::new();
        
        for (index, projectile) in &mut game_struct.projectiles.iter_mut().enumerate() {

            match projectile {

                Projectiles::VarShot000(shot) => {

                    shot.update();

                    if !game_struct.canvas_rect.has_intersection(shot.rect) {
                        indices_to_remove.push(index);
                    }

                }

            }

        }

        // important to use pop here so we always remove indices from
        // right to left of vector
        while let Some(index) = indices_to_remove.pop() {
            game_struct.projectiles.swap_remove(index);
        }

        Ok(())
    }

    fn draw(&self, canvas: &mut WindowCanvas, game_struct: &GameStruct) -> Result<(), String> {

        canvas.set_draw_color(Color::RGB(100, 100, 100));
        canvas.clear();

        for struct2d in &game_struct.actors {

            match struct2d {

                Actors::VarEnemyA(ea) => 

                    canvas.copy(
                        &ea.texture,
                        None,
                        Some(ea.rect),
                    )?


            }

        }

        for struct2d in &game_struct.projectiles {

            match struct2d {

                Projectiles::VarShot000(shot) => 

                    canvas.copy(
                        &shot.texture,
                        None,
                        Some(shot.rect),
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
