
use std::collections::HashMap;

use sdl2::render::{Texture, TextureQuery};
use sdl2::rect::Rect;

use crate::gamestruct::GameStruct;


pub struct Player<'a> {
    pub texture: &'a Texture<'a>,
    pub rect: Rect,
}

impl<'a> Player<'a> {

    pub fn new(texture_map: &'a HashMap<String, Texture>) -> Result<Self, String> {

        let key_string = "ship_0000.png".to_string();
        let texture = &texture_map.get(&key_string).unwrap();

        let TextureQuery {width, height, .. } = texture.query();

        let rect = Rect::new(100, 100, width, height);

        Ok(
            Self{texture: &texture, rect: rect}
        )

    }

    pub fn shoot(
        &self,
        game_struct: &mut GameStruct,
        texture_map: &'a HashMap<String, Texture>
    ) {

        game_struct.request_shoot(
            (self.rect.center().x(), self.rect.y()),
            &texture_map,
        );

    }

}
