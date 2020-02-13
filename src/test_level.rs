/// make a level to test many things

use {
    crate::{
        actor::*,
        consts::*,
        board::Board,
        item::*,
        pos::*,
    },
};

pub fn build() -> Board {
    let area = PosArea {
        min: Pos::new(-10, -10),
        dim: Pos::new(60, 60),
    };
    let mut board = Board::new(area, FIELD);
    board.actors[0].pos = Pos::new(30, 10);

    board.cells.set_xy(2, 3, WALL);
    board.set_h_line(6..17, 4, WALL);
    board.set_h_line(8..37, 8, WALL);
    board.cells.set_xy(6, 5, WALL);
    board.set_range(5..12, 0..2, FOREST);
    board.set_v_line(27, 0..4, WALL);
    board.set_h_line(28..36, 0, WALL);

    board.set_v_line(46, 0..12, WATER);
    board.set_v_line(46, 13..26, WATER);
    board.set_range(35..46, 23..26, WATER);

    // chateau du chevalier
    board.set_range(0..35, 19..26, WATER);
    board.set_range(2..9, 20..25, WALL);
    board.set_range(3..8, 21..24, FIELD);
    board.set_h_line(8..36, 22, FIELD);
    board.add_actor_in(ActorKind::Knight, 4, 22);

    // pont sud
    board.set_range(40..43, 23..26, FIELD);

    board.add_actor_in(ActorKind::Hunter, 15, -3);
    board.add_actor_in(ActorKind::Hunter, 70, 20);
    board.add_actor_in(ActorKind::Wolf, 17, 30);
    board.add_actor_in(ActorKind::Fox, 50, 5);
    board.add_actor_in(ActorKind::Fox, 40, 16);
    board.add_actor_in(ActorKind::Fox, 4, 17);

    // test animate
    //board.add_actor_in(ActorKind::Fox, 40, 17);
    //board.add_actor_in(ActorKind::Fox, 41, 16);

    board.add_item_in(ItemKind::Carrot, 31, 12);
    board.add_item_in(ItemKind::Carrot, 31, 13);
    board.add_item_in(ItemKind::Carrot, 33, 14);
    board.add_item_in(ItemKind::Carrot, 23, 11);
    board.add_item_in(ItemKind::Carrot, 27, 22);
    board.add_item_in(ItemKind::Carrot, 33, 22);
    for x in -5..-1 {
        for y in -7..-4 {
            board.add_item_in(ItemKind::Carrot, x, y);
        }
    }
    for x in 55..61 {
        for y in 7..9 {
            board.add_item_in(ItemKind::Carrot, x, y);
        }
    }
    board
}

