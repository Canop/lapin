/// make a level to test many things
///

use {
    crate::{
        actor::*,
        consts::*,
        board::Board,
        pos::*,
    },
};

pub fn build() -> Board {
    let mut board = Board::new(60, 50);
    board.actors[0].pos = Pos::new(30, 10);
    board.set_at(2, 3, WALL);
    board.set_h_line(6..17, 4, WALL);
    board.set_h_line(8..37, 8, WALL);
    board.set_at(6, 5, WALL);
    board.set_range(5..12, 0..2, FOREST);

    board.set_v_line(46, 0..12, WATER);
    board.set_v_line(46, 13..26, WATER);
    board.set_range(35..46, 23..26, WATER);

    // chateau du chevalier
    board.set_range(0..35, 19..26, WATER);
    board.set_range(2..9, 20..25, WALL);
    board.set_range(3..8, 21..24, VOID);
    board.set_h_line(8..36, 22, VOID);
    board.add_in(Kind::Knight, 4, 22);

    board.set_range(40..43, 23..26, VOID);

    board.add_in(Kind::Fox, 50, 5);
    board.add_in(Kind::Fox, 40, 16);
    board.add_in(Kind::Fox, 4, 17);
    board
}

