use {
    anyhow::Result,
    crossterm::{
        cursor,
        terminal::{Clear, ClearType},
        QueueableCommand,
    },
    std::io::Write,
    super::*,
    termimad::Area,
};

pub struct Screen {
    area: Area, // the complete screen
    pub areas: Areas, // the areas of the different sub parts
    layout: Layout,
}

impl Screen {
    pub fn new(layout: Layout) -> Screen {
        let area = Area::full_screen();
        let areas = layout.compute(&area);
        Self {
            area,
            areas,
            layout,
        }
    }
    pub fn set_terminal_size(&mut self, w: u16, h: u16) {
        self.area = Area::new(0, 0, w, h);
        self.areas = self.layout.compute(&self.area);
    }
    /// move the cursor to x,y and clears the line.
    pub fn goto_clear(&self, w: &mut impl Write, x: u16, y: u16) -> Result<()> {
        self.goto(w, x, y)?;
        self.clear_line(w)
    }
    /// move the cursor to x,y
    pub fn goto(&self, w: &mut impl Write, x: u16, y: u16) -> Result<()> {
        w.queue(cursor::MoveTo(x, y))?;
        Ok(())
    }
    /// clear the whole screen
    pub fn clear(&self, w: &mut impl Write) -> Result<()> {
        w.queue(Clear(ClearType::All))?;
        Ok(())
    }
    /// clear from the cursor to the end of line
    pub fn clear_line(&self, w: &mut impl Write) -> Result<()> {
        w.queue(Clear(ClearType::UntilNewLine))?;
        Ok(())
    }
}
