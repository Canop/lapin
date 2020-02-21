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
pub enum PenShape {
    Dot,
    Line,
    Rect,
}
pub static PEN_SHAPES: &'static[PenShape] = &[PenShape::Dot, PenShape::Line, PenShape::Rect];

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
    pub shape: PenShape,
    pub ink: PenInk,
    shape_start: Option<Pos>,
}

impl Default for Pen {
    fn default() -> Self {
        Self {
            shape: PenShape::Dot,
            ink: PenInk::Terrain(FIELD),
            shape_start: None,
        }
    }
}

impl Pen {
    fn ink_pos(&self, pos: Pos, board: &mut Board) {
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
        match self.shape {
            PenShape::Dot => {
                self.ink_pos(click_pos, board);
            }
            PenShape::Line => {
                if let Some(Pos { mut x, mut y }) = self.shape_start {
                    // based on Bresenham's algorithm.
                    // Prevents crossing in quadrant dirs (diagonal possible)
                    let Pos {x: xf, y: yf} = click_pos;
                    let dx = (xf - x).abs();
                    let sx = if x<xf { 1 } else { -1 };
                    let dy = -(yf - y).abs();
                    let sy = if y<yf { 1 } else { -1 };
                    let mut e = dx + dy;
                    loop {
                        self.ink_pos(Pos::new(x, y), board);
                        if x==xf && y==yf {
                            break;
                        }
                        let e2 = 2*e;
                        if e2 >= dy {
                            e += dy;
                            x += sx;
                        }
                        if e2 <= dx {
                            e += dx;
                            y += sy;
                        }
                    }
                    self.shape_start = None;
                } else {
                    self.shape_start = Some(click_pos);
                }
            }
            PenShape::Rect => {
                if let Some(start) = self.shape_start {
                    for x in start.x.min(click_pos.x)..=start.x.max(click_pos.x) {
                        for y in start.y.min(click_pos.y)..=start.y.max(click_pos.y) {
                            self.ink_pos(Pos::new(x, y), board);
                        }
                    }
                    self.shape_start = None;
                } else {
                    self.shape_start = Some(click_pos);
                }
            }
        }
    }
    pub fn set_ink(&mut self, ink: PenInk) {
        self.ink = ink;
        debug!("new pen ink: {:?}", self.ink);
    }
    pub fn set_shape(&mut self, shape: PenShape) {
        self.shape = shape;
        self.shape_start = None;
    }
    pub fn shape_started(&self) -> bool {
        self.shape_start.is_some()
    }
}





