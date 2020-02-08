use {
    anyhow::Result,
    crate::{
        io::W,
    },
    crossterm::{
        cursor,
        QueueableCommand,
    },
    termimad::{
        Area,
    },
};

pub type Int = i16;

// a position in the real world (the one full of rabbits and wolves)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Pos {
    pub x: Int,
    pub y: Int,
}

#[derive(Debug, Clone, Copy)]
pub enum Dir {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug, Clone, Copy)]
pub struct ScreenPos {
    pub x: u16,
    pub y: u16,
}

impl Pos {
    pub fn center_of(area: &Area) -> Self {
        Self {
            x: (area.left+area.width/2) as Int,
            y: (area.top+area.height/2) as Int,
        }
    }
    pub fn in_grid(self, width: Int, height: Int) -> bool {
        self.x>=0 && self.y>=0 && self.x<width && self.y<height
    }
    pub fn to_screen(self, lapin_pos: Pos, area: &Area) -> Option<ScreenPos> {
        let (w, h) = (area.width as Int, area.height as Int);
        let x = (self.x - lapin_pos.x)  + w / 2;
        let y = (self.y - lapin_pos.y)  + h / 2;
        if x>=0 && y>=0 && x<w && y<h {
            let x = x as u16;
            let y = y as u16;
            Some(ScreenPos {
                x: x - 1 + area.left,
                y: y - 1 + area.top,
            })
        } else {
            None
        }
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
            _ => None,
        }
    }
    pub fn in_dir(&self, dir: Dir) -> Self {
        match dir {
            Dir::Up => Pos { x:self.x, y:self.y-1 },
            Dir::Right => Pos { x:self.x+1, y:self.y },
            Dir::Down => Pos { x:self.x, y:self.y+1 },
            Dir::Left => Pos { x:self.x-1, y:self.y },
        }
    }
}


impl ScreenPos {
    pub fn goto(self, w: &mut W) -> Result<()> {
        w.queue(cursor::MoveTo(self.x, self.y))?;
        Ok(())
    }
}

pub trait Mobile {
    fn get_pos(&self) -> Pos;
    fn set_pos(&mut self, pos: Pos) -> Pos;
}