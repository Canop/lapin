use {
    crate::{
        actor::*,
        board::Board,
        item::*,
        pos::*,
        terrain::*,
    },
    serde::{Serialize, Deserialize},
};

/// the description of a level for (de)serialization
/// and edition (but not game nor edition)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Level {
    pub name: String,
    pub default_terrain: Terrain,
    pub terrains: Vec<Located<Terrain>>,
    pub actors: Vec<Actor>, // actors[0] must be the Lapin
    pub items: Vec<Located<Item>>,
}

impl Default for Level {
    fn default() -> Self {
        Self {
            name: String::new(),
            default_terrain: Terrain::Mud,
            terrains: Vec::new(),
            actors: vec![Actor::new(ActorKind::Lapin, 0, 0)],
            items: Vec::new(),
        }
    }
}

impl From<&Board> for Level {
    fn from(board: &Board) -> Self {
        let mut level = Level::default();
        level.name = board.name.clone();
        level.default_terrain = board.terrains.default;
        level.actors = board.actors.clone();
        for lc in board.terrains.iter() {
            if lc.v != level.default_terrain {
                level.terrains.push(lc);
            }
        }
        for lc in board.items.iter_some() {
            level.items.push(lc);
        }
        level
    }
}


