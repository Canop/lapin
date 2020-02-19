use {
    crate::{
        actor::*,
        board::*,
        consts::*,
        item::*,
        pos::*,
    },
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PenMode {
    Char,
    Line,
    Rect,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PenInk {
    EraseTerrain,
    Terrain(Cell),
    EraseItem,
    Item(ItemKind),
    EraseActor,
    Actor(ActorKind),
}


/// defines what will happen on click on the board
#[derive(Debug, Clone, Copy)]
pub struct Pen {
    pub mode: PenMode,
    pub ink: PenInk,
}

impl Default for Pen {
    fn default() -> Self {
        Self {
            mode: PenMode::Char,
            ink: PenInk::Terrain(FIELD),
        }
    }
}

impl Pen {
    fn apply_pos(&self, pos: Pos, board: &mut Board) {
        match self.ink {
            PenInk::EraseTerrain => {
                board.cells.unset(pos);
            }
            PenInk::Terrain(cell) => {
                board.cells.set(pos, cell);
            }
            PenInk::EraseItem => {
                board.items.remove(pos);
            }
            PenInk::Item(item_kind) => {
                board.add_item_in(item_kind, pos.x, pos.y);
            }
            PenInk::EraseActor => {
                board.actors.retain(|&actor| actor.pos != pos);
            }
            PenInk::Actor(actor_kind) => {
                if actor_kind == ActorKind::Lapin {
                    // we're just moving THE lapin
                    board.actors[0].pos = pos;
                    return;
                }
                // we check we're not removing the lapin
                if pos == board.lapin_pos() {
                    return; // we make it a no-op
                }
                // we must now ensure any previous actor in pos is removed
                board.actors.retain(|&actor| actor.pos != pos);
                // and now we add the new actor
                board.add_actor_in(actor_kind, pos.x, pos.y);
            }
        }
    }
    pub fn click(&mut self, click_pos: Pos, board: &mut Board) {
        self.apply_pos(click_pos, board);
    }
    pub fn set_ink(&mut self, ink: PenInk) {
        self.ink = ink;
        debug!("new pen ink: {:?}", self.ink);
    }
}





