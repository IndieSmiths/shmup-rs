
use std::path::Path;
use std::collections::HashMap;

use sdl2::render::{Texture, TextureCreator};
use sdl2::video::WindowContext;
use sdl2::image::LoadTexture;


pub fn get_texture_map<'a> (texture_creator: &'a TextureCreator<WindowContext>) -> HashMap<String, Texture<'a>> {

    let mut texture_map: HashMap<String, Texture> = HashMap::new();

    let directory = Path::new(file!())
                        .parent()
                        .unwrap()
                        .join("data")
                        .join("images");

    for entry in directory.read_dir().expect("read_dir call failed") {

        if let Ok(entry) = entry {
            
            let path = entry.path();
            
            if path.is_file() {

                if let Some(filename) = path.file_name() {

                    if let Some(a_str) = filename.to_str() {

                        let a_string = a_str.to_string();
                        let texture = texture_creator.load_texture(path).unwrap();

                        texture_map.insert(a_string, texture);

                    }

                }

            }

        }

    }

    texture_map

}
