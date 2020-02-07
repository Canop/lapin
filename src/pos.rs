use {
    anyhow::Result,
    crate::{
        io::W,
    },
    crossterm::{
        cursor,
        QueueableCommand,
    },
    std::io::Write,
    termimad::{
        Area,
    },
};

pub type Int = i16;

// a position in the real world (the one full of rabbits and wolves)
#[derive(Debug, Clone, Copy)]
pub struct Pos {
    pub x: Int,
    pub y: Int,
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
}

#[derive(Debug, Clone, Copy)]
pub struct ScreenPos {
    pub x: u16,
    pub y: u16,
}

impl ScreenPos {
    pub fn goto(self, w: &mut W) -> Result<()> {
        w.queue(cursor::MoveTo(self.x, self.y))?;
        Ok(())
    }
}

