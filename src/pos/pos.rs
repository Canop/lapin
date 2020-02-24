use {
    serde::{Serialize, Deserialize},
    super::*,
};

// a position in the real world (the one full of rabbits and wolves)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub struct Pos {
    pub x: Int,
    pub y: Int,
}

impl Pos {
    pub fn new(x: Int, y: Int) -> Self {
        Self { x, y }
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
    pub fn first_dir(self, path: &[Pos]) -> Option<Dir> {
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
    /// return major then minor among NESW
    pub fn quadrants_to(&self, dst:Pos) -> [Dir; 2] {
        use Dir::*;
        let (dx, dy) = (dst.x-self.x, dst.y-self.y);
        if dx.abs() < dy.abs() {
            if dy < 0 {
                [Up, if dx < 0 { Left } else { Right }]
            } else {
                [Down, if dx < 0 { Left } else { Right }]
            }
        } else {
            if dx > 0 {
                [Right, if dy < 0 { Up } else { Down }]
            } else {
                [Left, if dy < 0 { Up } else { Down }]
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



