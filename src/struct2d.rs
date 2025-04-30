
use std::collections::HashMap;

use sdl2::render::{Texture, TextureQuery};
use sdl2::rect::Rect;


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

        Ok(Self{ texture: &texture, rect: rect})

    }
}


// ### TODO code below may be useful in next steps
//
//pub struct EnemyA<'a> {
//    texture: &'a Texture<'a>,
//    rect: Rect,
//}
//
//impl<'a> EnemyA<'a> {
//
//    pub fn new(
//        coordinates_name: &str,
//        coordinates_value: &str,
//        texture_map: &'a HashMap<String, Texture>,
//    ) -> Result<Self, String> {
//
//        let key_string = "ship_0000.png".to_string();
//        let texture = &texture_map.get(&key_string).unwrap();
//
//        let TextureQuery {width, height, .. } = texture.query();
//
//        let rect = Rect::new(0, 0, width, height);
//
//        Ok(Self {texture: &texture, rect: rect}})
//
//    }
//}



pub enum Struct2D<'a> {
    PlayerVar(Player<'a>),
    //EnemyAVar(EnemyA),
}
