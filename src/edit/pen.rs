use {
    crate::{
        board::*,
        consts::*,
        pos::*,
    },
};


/// defines what will happen on click on the board
#[derive(Debug, Clone, Copy)]
pub struct Pen {
    cell: Cell,
}

impl Default for Pen {
    fn default() -> Self {
        Self {
            cell: WALL,
        }
    }
}

impl Pen {
    pub fn click(&mut self, pos: Pos, board: &mut Board) {
        board.set(pos, self.cell);
    }
}





