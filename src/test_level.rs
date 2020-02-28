/// make a level to test many things

use {
    crate::{
        actor::ActorKind::*,
        board::Board,
        item::ItemKind::*,
        level::Level,
        pos::*,
        terrain::Terrain::*,
    },
};

pub fn build() -> Level {
    let mut board = Board::new(
        "test level".to_string(),
        PosArea::new(-20..100, -20..100),
        Mud,
    );
    board.actors[0].pos = Pos::new(30, 10);

    board.set_v_line(2, 3..5, Stone);
    board.set_h_line(6..17, 4, Stone);
    board.set_h_line(8..37, 8, Stone);
    board.set_range(5..8, 0..2, Grass);
    board.set_v_line(27, 0..4, Stone);
    board.set_h_line(28..36, 0, Stone);

    board.set_v_line(46, 0..12, Water);
    board.set_v_line(46, 13..26, Water);
    board.set_range(35..46, 23..26, Water);

    // chateau du chevalier
    board.set_range(0..35, 19..26, Water);
    board.set_range(2..9, 20..25, Stone);
    board.set_range(3..8, 21..24, Mud);
    board.set_h_line(8..36, 22, Mud);
    board.add_actor_in(Knight, 4, 22);

    // pont sud
    board.set_range(40..43, 23..26, Mud);

    board.add_actor_in(Hunter, 12, -3);
    board.add_actor_in(Hunter, 70, 20);
    board.add_actor_in(Hunter, -11, 23);
    board.add_actor_in(Wolf, 27, 30);
    board.add_actor_in(Fox, 50, 5);
    board.add_actor_in(Fox, 40, 16);
    board.add_actor_in(Fox, 4, 17);
    board.add_actor_in(Sheep, -4, 21);
    for x in 50..53 {
        for y in 0..2 {
            board.add_actor_in(Sheep, x, y);
        }
    }

    board.add_item_in(Wine, 11, 3);
    board.add_item_in(Wine, 28, -3);
    board.add_item_in(Carrot, 31, 12);
    board.add_item_in(Carrot, 31, 13);
    board.add_item_in(Carrot, 33, 14);
    board.add_item_in(Carrot, 23, 11);
    board.add_item_in(Carrot, 27, 22);
    board.add_item_in(Carrot, 33, 22);
    for x in -5..-1 {
        for y in -7..-4 {
            board.add_item_in(Carrot, x, y);
        }
    }
    for x in 55..61 {
        for y in 7..9 {
            board.add_item_in(Carrot, x, y);
        }
    }
    Level::from(&board)
}

