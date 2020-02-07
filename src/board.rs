use crate::{
    command::*,
    consts::*,
    pos::*,
    lapin::Lapin,
};

/// the game state
pub struct Board {
    pub width: Int,
    pub height: Int,
    grid: Vec<Cell>,
    pub lapin: Lapin,
}

impl Board {
    pub fn new(width: usize, height: usize) -> Self {
        let grid = vec![VOID; width * height];
        let lapin = Lapin::new(10, 10);
        Self {
            width: width as Int,
            height: height as Int,
            grid,
            lapin,
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

    pub fn apply(&mut self, cmd: Command) {
        match cmd {
            Command::Move(Dir::Up) => {
                self.lapin.pos.y -= 1;
            }
            Command::Move(Dir::Right) => {
                self.lapin.pos.x += 1;
            }
            Command::Move(Dir::Down) => {
                self.lapin.pos.y += 1;
            }
            Command::Move(Dir::Left) => {
                self.lapin.pos.x -= 1;
            }
            _ => {
                debug!("a pa capito");
            }
        }
    }
}
