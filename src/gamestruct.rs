
use std::collections::{HashMap, HashSet};
use std::time::Instant;

use sdl2::keyboard::Scancode;
use sdl2::render::Texture;
use sdl2::rect::Rect;

use crate::struct2d::actor::Actors;
use crate::struct2d::projectile::{Projectiles, Shot000};


pub struct GameStruct<'a> {
    pub canvas_rect: Rect,
    pub actors: Vec<Actors<'a>>,
    pub projectiles: Vec<Projectiles<'a>>,
    pub pressed_keys: HashSet<Scancode>,
    pub shoot_cooldown: Option<Instant>,
}


impl<'a> GameStruct<'a> {

    pub fn new ((canvas_width, canvas_height): (u32, u32)) -> Self {

        Self {
            canvas_rect: Rect::new(0, 0, canvas_width, canvas_height),
            actors: Vec::new(),
            projectiles: Vec::new(),
            pressed_keys: HashSet::new(),
            shoot_cooldown: None,
        }

    }

    pub fn update(&mut self) {

        if let Some(instant) = self.shoot_cooldown {

            if instant.elapsed().as_millis() > 300 {
                self.shoot_cooldown = None;
            }

        }

    }

    pub fn request_shoot(
        &mut self,
        midbottom: (i32, i32),
        texture_map: &'a HashMap<String, Texture<'a>>,
    ) {

        if let None = self.shoot_cooldown {

            let coordinates_name = "midbottom";
            let shot = Shot000::new(&coordinates_name, midbottom, texture_map).unwrap();
            let shotvar = Projectiles::VarShot000(shot);

            self.projectiles.push(shotvar);

            self.shoot_cooldown = Some(Instant::now());

        }
    }

}
