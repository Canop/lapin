
use crate::{
    pos::*,
};

pub struct Knight {
    pub pos: Pos,
}

impl Knight {
    pub fn new(x: Int, y: Int) -> Self {
        let pos = Pos { x, y };
        Self {
            pos,
        }
    }
}

