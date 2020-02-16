use {
    crate::{
        actor::*,
        board::Board,
        consts::*,
        item::*,
        pos::*,
    },
    fnv::{
        FnvHashMap,
    },
    serde::{Serialize, Deserialize},
    std::{
        ops::{
            Range,
        },
    },
};

/// the description of a level for (de)serialization
/// and edition (but not game)
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Level {
    pub default_cell: Cell,
    pub cells: Vec<Located<Cell>>,
    pub actors: Vec<Actor>, // actors[0] must be the Lapin
    pub items: Vec<Located<Item>>,
}

impl From<&Board> for Level {
    fn from(board: &Board) -> Self {
        let mut level = Level::default();
        level.default_cell = board.cells.default;
        level.actors.extend(&board.actors);
        for lc in board.cells.iter() {
            if lc.v != level.default_cell {
                level.cells.push(lc);
            }
        }
        for lc in board.items.iter_some() {
            level.items.push(lc);
        }
        level
    }
}


