use {
    crate::{
        actor::*,
        board::Board,
        consts::*,
        item::*,
        pos::*,
    },
    serde::{Serialize, Deserialize},
};

/// the description of a level for (de)serialization
/// and edition (but not game)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Level {
    pub default_cell: Cell,
    pub cells: Vec<Located<Cell>>,
    pub actors: Vec<Actor>, // actors[0] must be the Lapin
    pub items: Vec<Located<Item>>,
}

impl Default for Level {
    fn default() -> Self {
        Self {
            default_cell: FIELD,
            cells: Vec::new(),
            actors: vec![Actor::new(ActorKind::Lapin, 0, 0)],
            items: Vec::new(),
        }
    }
}

impl From<&Board> for Level {
    fn from(board: &Board) -> Self {
        let mut level = Level::default();
        level.default_cell = board.cells.default;
        level.actors = board.actors.clone();
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


