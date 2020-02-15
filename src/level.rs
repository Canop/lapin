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
    pub lapin_pos: Pos,
    pub cells: FnvHashMap<Pos, Cell>,
    pub actors: FnvHashMap<Pos, Actor>,
    pub items: FnvHashMap<Pos, Item>,
}

impl Level {

    pub fn set_xy(&mut self, x: Int, y: Int, cell: Cell) {
        self.cells.insert(Pos::new(x, y), cell);
    }
    pub fn set(&mut self, pos: Pos, cell: Cell) {
        self.cells.insert(pos, cell);
    }
    pub fn set_range(&mut self, rx: Range<Int>, ry: Range<Int>, cell: Cell) {
        for x in rx {
            for y in ry.clone() {
                self.set(Pos::new(x, y), cell);
            }
        }
    }
    pub fn set_h_line(&mut self, rx: Range<Int>, y: Int, cell: Cell) {
        self.set_range(rx, y..y+1, cell);
    }
    pub fn set_v_line(&mut self, x: Int, ry: Range<Int>, cell: Cell) {
        self.set_range(x..x+1, ry, cell);
    }
    pub fn get(&self, pos: Pos) -> Cell {
        if let Some(c) = self.cells.get(&pos) {
            *c
        } else {
            self.default_cell
        }
    }
    pub fn add_actor_in(&mut self, kind: ActorKind, x: Int, y: Int) {
        self.actors.insert(Pos::new(x, y), Actor::new(kind, x, y));
    }
    pub fn add_item_in(&mut self, kind: ItemKind, x: Int, y: Int) {
        self.items.insert(Pos::new(x, y), Item { kind });
    }

    pub fn to_board(self) -> Board {
        let pos_distribution = PosDistribution::from(self.cells.keys())
            .unwrap_or_default();
        // FIXME check area not to wide
        let mut board = Board::new(pos_distribution.area, self.default_cell);
        board.actors[0].pos = self.lapin_pos;
        for (pos, cell) in self.cells {
            board.set(pos, cell);
        }
        for (pos, mut actor) in self.actors {
            actor.pos = pos;
            board.add_actor(actor);
        }
        for (pos, item) in self.items {
            board.items.set_some(pos, item);
        }

        board
    }
}

