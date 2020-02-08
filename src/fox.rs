use crate::{
    pos::*,
};

pub struct Fox {
    pub pos: Pos,
}

impl Fox {
    pub fn new(x: Int, y: Int) -> Self {
        let pos = Pos { x, y };
        Self {
            pos,
        }
    }
}

