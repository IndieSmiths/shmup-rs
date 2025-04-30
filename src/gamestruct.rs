

//use crate::struct2d::Struct2D;


pub struct GameStruct {
    pub actors: Vec<i32>,
}

impl GameStruct {

    pub fn new () -> Self {
        Self {
            actors: vec![1, 2, 3],
        }
    }

}
