/// make a level to test many things

use {
    crate::{
        actor::*,
        consts::*,
        board::Board,
        item::*,
        level::Level,
        pos::*,
    },
};

pub fn build() -> Level {
    let mut level = Level::default();
    level.lapin_pos = Pos::new(30, 10);

    level.set_xy(2, 3, WALL);
    level.set_h_line(6..17, 4, WALL);
    level.set_h_line(8..37, 8, WALL);
    level.set_xy(6, 5, WALL);
    level.set_range(5..8, 0..2, GRASS);
    level.set_v_line(27, 0..4, WALL);
    level.set_h_line(28..36, 0, WALL);

    level.set_v_line(46, 0..12, WATER);
    level.set_v_line(46, 13..26, WATER);
    level.set_range(35..46, 23..26, WATER);

    // chateau du chevalier
    level.set_range(0..35, 19..26, WATER);
    level.set_range(2..9, 20..25, WALL);
    level.set_range(3..8, 21..24, FIELD);
    level.set_h_line(8..36, 22, FIELD);
    level.add_actor_in(ActorKind::Knight, 4, 22);

    // pont sud
    level.set_range(40..43, 23..26, FIELD);

    level.add_actor_in(ActorKind::Hunter, 12, -3);
    level.add_actor_in(ActorKind::Hunter, 70, 20);
    level.add_actor_in(ActorKind::Wolf, 27, 30);
    level.add_actor_in(ActorKind::Fox, 50, 5);
    level.add_actor_in(ActorKind::Fox, 40, 16);
    level.add_actor_in(ActorKind::Fox, 4, 17);
    level.add_actor_in(ActorKind::Sheep, -4, 21);
    for x in 50..53 {
        for y in 0..2 {
            level.add_actor_in(ActorKind::Sheep, x, y);
        }
    }

    level.add_item_in(ItemKind::Wine, 11, 3);
    level.add_item_in(ItemKind::Wine, 28, -3);
    level.add_item_in(ItemKind::Carrot, 31, 12);
    level.add_item_in(ItemKind::Carrot, 31, 13);
    level.add_item_in(ItemKind::Carrot, 33, 14);
    level.add_item_in(ItemKind::Carrot, 23, 11);
    level.add_item_in(ItemKind::Carrot, 27, 22);
    level.add_item_in(ItemKind::Carrot, 33, 22);
    for x in -5..-1 {
        for y in -7..-4 {
            level.add_item_in(ItemKind::Carrot, x, y);
        }
    }
    for x in 55..61 {
        for y in 7..9 {
            level.add_item_in(ItemKind::Carrot, x, y);
        }
    }
    level
}

