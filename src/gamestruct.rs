

use crate::struct2d::Struct2D;


pub struct GameStruct<'a> {
    pub actors: Vec<Struct2D<'a>>,
}

impl GameStruct<'_> {

    pub fn new () -> Self {

        Self {
            actors: Vec::new(),
        }

    }

}
