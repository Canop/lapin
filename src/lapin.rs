use crate::{
    pos::*,
};


pub struct Lapin {
    pub pos: Pos,
}

impl Lapin {
    pub fn new(x: Int, y: Int) -> Self {
        let pos = Pos { x, y };
        Self {
            pos,
        }
    }
}
