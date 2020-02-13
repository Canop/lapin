use {
    anyhow::Result,
    crate::{
        io::W,
        screen::Screen,
    },
    crossterm::{
        cursor,
        QueueableCommand,
    },
    termimad::{
        Area,
    },
};

pub type Int = i32;

// a position in the real world (the one full of rabbits and wolves)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Pos {
    pub x: Int,
    pub y: Int,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Dir {
    Up,
    Right,
    Down,
    Left,
    UpRight,
    RightDown,
    DownLeft,
    LeftUp
}

#[derive(Debug, Clone, Copy)]
pub struct ScreenPos {
    pub x: u16,
    pub y: u16,
}

impl Pos {
    pub fn new(x: Int, y: Int) -> Self {
        Self { x, y }
    }
    pub fn center_of(area: &Area) -> Self {
        Self {
            x: (area.left+area.width/2) as Int,
            y: (area.top+area.height/2) as Int,
        }
    }
    pub fn in_grid(self, width: Int, height: Int) -> bool {
        self.x>=0 && self.y>=0 && self.x<width && self.y<height
    }
    pub fn mh_distance(a: Pos, b: Pos) -> Int {
        (a.x-b.x).abs().max((a.y-b.y).abs())
    }
    pub fn manhattan_distance(a: Pos, b: Pos) -> Int {
        (a.x-b.x).abs() + (a.y-b.y).abs()
    }
    /// return the first direction to follow on a path
    /// (or none if we're yet on destination or if the
    /// path doesn't starts from there)
    pub fn first_dir(&self, path: &Vec<Pos>) -> Option<Dir> {
        path.get(0).and_then(|dst| self.dir_to(*dst))
    }
    /// return the direction to follow to directly reach
    /// the dst. Return None if the other pos isn't a
    /// direct neighbour.
    pub fn dir_to(&self, dst: Pos) -> Option<Dir> {
        match (dst.x-self.x, dst.y-self.y) {
            (0, -1) => Some(Dir::Up),
            (1, 0)  => Some(Dir::Right),
            (0, 1)  => Some(Dir::Down),
            (-1, 0) => Some(Dir::Left),
            (1, -1) => Some(Dir::UpRight),
            (1, 1)  => Some(Dir::RightDown),
            (-1, 1)  => Some(Dir::DownLeft),
            (-1, -1) => Some(Dir::LeftUp),
            _ => None,
        }
    }
    pub fn quadrant_to(&self, dst:Pos) -> Dir {
        let (dx, dy) = (dst.x-self.x, dst.y-self.y);
        if dx.abs() < dy.abs() {
            if dy < 0 {
                Dir::Up
            } else {
                Dir::Down
            }
        } else {
            if dx > 0 {
                Dir::Right
            } else {
                Dir::Left
            }
        }
    }
    pub fn is_in_dir(&self, dst: Pos, dir: Dir) -> bool {
        let (dx, dy) = (dst.x-self.x, dst.y-self.y);
        match dir {
            Dir::Up => dx == 0 && dy < 0,
            Dir::Right => dy == 0 && dx > 0,
            Dir::Down => dx == 0 && dy > 0,
            Dir::Left => dy == 0 && dx < 0,
            _ => {
                warn!("not implemented");
                false
            }
        }
    }
    pub fn in_dir(&self, dir: Dir) -> Self {
        match dir {
            Dir::Up => Pos { x:self.x, y:self.y-1 },
            Dir::Right => Pos { x:self.x+1, y:self.y },
            Dir::Down => Pos { x:self.x, y:self.y+1 },
            Dir::Left => Pos { x:self.x-1, y:self.y },
            Dir::UpRight => Pos { x:self.x+1, y:self.y-1 },
            Dir::RightDown => Pos { x:self.x+1, y:self.y+1 },
            Dir::DownLeft => Pos { x:self.x-1, y:self.y+1 },
            Dir::LeftUp => Pos { x:self.x-1, y:self.y-1 },
        }
    }
}

impl ScreenPos {
    pub fn new(x: u16, y:u16) -> Self {
        Self { x, y }
    }
    pub fn goto(self, w: &mut W) -> Result<()> {
        w.queue(cursor::MoveTo(self.x, self.y))?;
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct PosArea {
    pub min: Pos,
    pub dim: Pos,
}
impl PosArea {
    pub fn contains(self, pos: Pos) -> bool {
        pos.x >= self.min.x
            && pos.y >= self.min.y
            && pos.x < self.min.x + self.dim.x
            && pos.y < self.min.y + self.dim.y
    }
    pub fn nearest(self, mut pos: Pos) -> Pos {
        if pos.x < self.min.x {
            pos.x = self.min.x;
        } else if pos.x >= self.min.x + self.dim.x {
            pos.x = self.min.x + self.dim.x -1;
        }
        if pos.y < self.min.y {
            pos.y = self.min.y;
        } else if pos.y >= self.min.y + self.dim.y {
            pos.y = self.min.y + self.dim.y -1;
        }
        pos
    }
}

#[derive(Debug, Clone, Copy)]
pub struct PosConverter {
    dec: Pos,
    dim: Pos,
}
impl PosConverter {
    pub fn from(center: Pos, screen: &Screen) -> Self {
        let dim = Pos {
            x: screen.board_area.width as Int,
            y: screen.board_area.height as Int,
        };
        let dec = Pos {
            x: dim.x / 2 - center.x,
            y: dim.y / 2 - center.y,
        };
        Self { dec, dim }
    }
    pub fn to_screen(&self, pos: Pos) -> Option<ScreenPos> {
        let x = pos.x + self.dec.x;
        let y = pos.y + self.dec.y;
        if x>=0 && y>=0 && x<self.dim.x && y<self.dim.y {
            Some(ScreenPos {
                x: x as u16,
                y: y as u16,
            })
        } else {
            None
        }
    }
    pub fn to_real(&self, sp: ScreenPos) -> Pos {
        let x = sp.x as Int - self.dec.x;
        let y = sp.y as Int - self.dec.y;
        Pos { x, y }
    }
}

