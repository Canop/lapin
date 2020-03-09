
use {
    crate::{
        core::*,
        pos::*,
    },
    super::{
        ink::*,
    },
};

/// an action which may be kept in a stack
/// for redoing from a saved board state
#[derive(Debug)]
pub enum DrawingAction {
    DotInk (Ink, Pos),
    LineInk (Ink, Pos, Pos),
    CompassLineInk (Ink, Pos, Pos), // a line in one of the 8 compass directions
    RectInk (Ink, Pos, Pos),
    DefaultTerrain(Terrain),
}

/// apply a drop of ink at some pos of the board.
/// Take care of keeping only one Lapin and only
/// one actor or item at most per terrain.
fn ink_pos(ink: Ink, pos: Pos, board: &mut Board) {
    match ink {
        Ink::EraseTerrain => {
            board.terrains.unset(pos);
        }
        Ink::Terrain(terrain) => {
            board.terrains.set(pos, terrain);
        }
        Ink::EraseItem => {
            board.items.remove(pos);
        }
        Ink::Item(item_kind) => {
            board.add_item_in(item_kind, pos.x, pos.y);
        }
        Ink::EraseActor => {
            board.actors.remove_by_pos(pos);
        }
        Ink::Actor(actor_kind) => {
            if actor_kind == ActorKind::Lapin {
                // we're just moving THE lapin
                board.actors.move_lapin_to(pos);
                return;
            }
            // we check we're not removing the lapin
            if pos == board.lapin_pos() {
                return; // we make it a no-op
            }
            // we must now ensure any previous actor in pos is removed
            board.actors.remove_by_pos(pos);
            // and now we add the new actor
            if let Err(e) = board.add_actor_in(actor_kind, pos.x, pos.y) {
                warn!("err in adding actor: {:?}", e);
            }
        }
    }
}

/// draw a line between two pos.
/// based on Bresenham's algorithm.
/// Prevents crossing in quadrant dirs (diagonal possible)
fn ink_line(ink: Ink, a: Pos, b: Pos, board: &mut Board) {
    let Pos {mut x, mut y} = a;
    let Pos {x: xf, y: yf} = b;
    let dx = (xf - x).abs();
    let sx = if x<xf { 1 } else { -1 };
    let dy = -(yf - y).abs();
    let sy = if y<yf { 1 } else { -1 };
    let mut e = dx + dy;
    loop {
        ink_pos(ink, Pos::new(x, y), board);
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
}

/// draw a line between two pos, forcing to one of the 8 main directions.
fn ink_compass_line(ink: Ink, a: Pos, b: Pos, board: &mut Board) {
    use Dir::*;
    let compass_dir = a.compass_to(b);
    debug!("compass_dir = {:?}", compass_dir);
    match compass_dir {
        Up => for y in b.y..=a.y {
            ink_pos(ink, Pos::new(a.x, y), board);
        }
        UpRight => for (i, x) in (a.x..=b.x).enumerate() {
            ink_pos(ink, Pos::new(x, a.y-i as i32), board);
        }
        Right => for x in a.x..=b.x {
            ink_pos(ink, Pos::new(x, a.y), board);
        }
        RightDown => for (i, x) in (a.x..=b.x).enumerate() {
            ink_pos(ink, Pos::new(x, a.y+i as i32), board);
        }
        Down => for y in a.y..=b.y {
            ink_pos(ink, Pos::new(a.x, y), board);
        }
        DownLeft => for (i, x) in (b.x..=a.x).rev().enumerate() {
            ink_pos(ink, Pos::new(x, a.y+i as i32), board);
        }
        Left => for x in b.x..=a.x {
            ink_pos(ink, Pos::new(x, a.y), board);
        }
        LeftUp => for (i, x) in (b.x..=a.x).rev().enumerate() {
            ink_pos(ink, Pos::new(x, a.y-i as i32), board);
        }
    }
}

/// fill a rect given two corners
fn ink_rect(ink: Ink, a: Pos, b: Pos, board: &mut Board) {
    for x in a.x.min(b.x)..=a.x.max(b.x) {
        for y in a.y.min(b.y)..=a.y.max(b.y) {
            ink_pos(ink, Pos::new(x, y), board);
        }
    }
}

impl DrawingAction {
    pub fn apply_to(&self, board: &mut Board) {
        time!(Debug, "draw act",
        match self {
            DrawingAction::DotInk(ink, pos) => {
                ink_pos(*ink, *pos, board);
            }
            DrawingAction::LineInk(ink, a, b) => {
                ink_line(*ink, *a, *b, board);
            }
            DrawingAction::CompassLineInk(ink, a, b) => {
                ink_compass_line(*ink, *a, *b, board);
            }
            DrawingAction::RectInk(ink, a, b) => {
                ink_rect(*ink, *a, *b, board);
            }
            DrawingAction::DefaultTerrain(terrain) => {
                board.terrains.default = *terrain;
            }
        }
        )
    }
}
