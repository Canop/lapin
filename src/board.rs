use crate::{
    command::*,
    consts::*,
    fox::Fox,
    lapin::Lapin,
    pos::*,
    world::*,
};

/// the game state
pub struct Board {
    pub width: Int,
    pub height: Int,
    grid: Vec<Cell>,
    pub lapin: Lapin,
    pub foxes: Vec<Fox>,
}

/// what we get on applying a world or player move
pub enum MoveResult {
    Ok, // RAS
    Invalid, // move does nothing,
    PlayerWin,
    PlayerLose,
}

impl Board {

    pub fn new(width: usize, height: usize) -> Self {
        let grid = vec![VOID; width * height];
        let lapin = Lapin::new(0, 0);
        Self {
            width: width as Int,
            height: height as Int,
            grid,
            lapin,
            foxes: Vec::new(),
        }
    }

    pub fn add_fox_in(&mut self, x: Int, y: Int) {
        self.foxes.push(Fox::new(x, y));
    }

    pub fn is_enterable(&self, pos: Pos) -> bool {
        match self.get(pos) {
            VOID => true,
            FOREST => true,
            _ => false,
        }
    }

    pub fn set_borders(&mut self, cell: Cell) {
        for x in 0..self.width {
            self.set_at(x, 0, cell);
            self.set_at(x, self.height-1, cell);
        }
        for y in 1..self.height-1 {
            self.set_at(0, y, cell);
            self.set_at(self.width-1, y, cell);
        }
    }

    pub fn idx(&self, pos: Pos) -> Option<usize> {
        if pos.in_grid(self.width, self.height) {
            Some((self.width * pos.y + pos.x) as usize)
        } else {
            None
        }
    }

    pub fn set(&mut self, pos: Pos, cell: Cell) {
        if let Some(idx) = self.idx(pos) {
            self.grid[idx] = cell;
        }
    }

    pub fn set_at(&mut self, x: Int, y: Int, cell: Cell) {
        if let Some(idx) = self.idx(Pos{x, y}) {
            self.grid[idx] = cell;
        }
    }

    pub fn get(&self, pos: Pos) -> Cell {
        match self.idx(pos) {
            Some(idx) => self.grid[idx],
            None => VOID,
        }
    }

    pub fn apply_player_move(&mut self, cmd: Command) -> MoveResult {
        match cmd {
            Command::Move(dir) => {
                let pos = self.lapin.pos.in_dir(dir);
                if self.is_enterable(pos) {
                    self.lapin.pos = pos;
                    if self.get(pos) == FOREST {
                        return MoveResult::PlayerWin;
                    }
                    for fox in &self.foxes {
                        if self.lapin.pos == fox.pos {
                            return MoveResult::PlayerLose;
                        }
                    }
                    MoveResult::Ok
                } else {
                    debug!("can't go there");
                    MoveResult::Invalid
                }
            }
            _ => {
                debug!("a pa capito");
                MoveResult::Invalid
            }
        }
    }

    pub fn apply_world_move(&mut self, world_move: WorldMove) -> MoveResult {
        for (fox, fox_move) in self.foxes.iter_mut().zip(world_move.fox_moves) {
            if let Some(dir) = fox_move {
                fox.pos = fox.pos.in_dir(dir);
                if self.lapin.pos == fox.pos {
                    return MoveResult::PlayerLose;
                }
            }
        }
        MoveResult::Ok
    }
}
