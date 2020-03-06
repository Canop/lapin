
use {
    anyhow::Result,
    crate::{
        display::W,
    },
    crossterm::{
        cursor,
        QueueableCommand,
    },
    termimad::{
        Area,
    },
};

/// a position on the screen, in characters counted from
/// top left corner of the terminal
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ScreenPos {
    pub x: u16,
    pub y: u16,
}

impl ScreenPos {
    pub fn new(x: u16, y:u16) -> Self {
        Self { x, y }
    }
    pub fn goto(self, w: &mut W) -> Result<()> {
        w.queue(cursor::MoveTo(self.x, self.y))?;
        Ok(())
    }
    pub fn is_in(self, area: &Area) -> bool {
        self.x >= area.left
            && self.x < area.left + area.width
            && self.y >= area.top
            && self.y < area.top + area.height
    }
}
