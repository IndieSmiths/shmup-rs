
use std::collections::HashMap;

use sdl2::render::{Texture, TextureQuery};
use sdl2::rect::Rect;

use crate::gamestruct::GameStruct;


pub struct Shot000<'a> {
    pub texture: &'a Texture<'a>,
    pub rect: Rect,
}

impl<'a> Shot000<'a> {

    pub fn new(
        coordinates_name: &str,
        coordinates_value: (i32, i32),
        texture_map: &'a HashMap<String, Texture>,
    ) -> Result<Self, String> {

        let key_string = "tile_0002.png".to_string();
        let texture = &texture_map.get(&key_string).unwrap();

        let TextureQuery {width, height, .. } = texture.query();

        let mut rect = Rect::new(0, 0, width, height);

        match coordinates_name {
            "topleft" => {
                rect.set_x(coordinates_value.0);
                rect.set_y(coordinates_value.1);
            }
            _ => rect.center_on(coordinates_value)
        }

        Ok(Self {texture: &texture, rect: rect})

    }

    pub fn update(&mut self) {
        self.rect.offset(0, -10);
    }
}


pub enum Projectiles<'a> {
    VarShot000(Shot000<'a>),
}
