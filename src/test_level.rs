/// make a level to test many things
///

use {
    crate::{
        consts::*,
        board::Board,
        pos::*,
    },
};

pub fn build() -> Board {
    let mut board = Board::new(60, 50);
    board.lapin.pos = Pos::new(30, 10);
    board.set_at(2, 3, WALL);
    for x in 6..17 {
        board.set_at(x, 4, WALL);
    }
    for x in 8..37 {
        board.set_at(x, 8, WALL);
    }
    board.set_at(6, 5, WALL);
    for x in 5..11 {
        board.set_at(x, 0, FOREST);
        board.set_at(x, 1, FOREST);
    }
    for y in 0..12 {
        board.set_at(46, y, WATER);
    }
    board.add_fox_in(50, 5);
    board.add_fox_in(40, 16);
    board.add_fox_in(4, 17);
    board
}

