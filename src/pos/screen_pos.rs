
use {
    anyhow::Result,
    crate::{
        io::W,
    },
    crossterm::{
        cursor,
        QueueableCommand,
    },
};

/// a position on the screen, in characters counted from
/// top left corner of the terminal
#[derive(Debug, Clone, Copy)]
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
}
