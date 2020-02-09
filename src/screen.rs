use {
    crate::skin::Skin,
    anyhow::Result,
    crossterm::{
        cursor,
        terminal::{Clear, ClearType},
        QueueableCommand,
    },
    std::io::Write,
    termimad::{Area},
};

pub struct Screen {
    pub width: u16,
    pub height: u16,
    pub board_area: Area,
    pub skin: Skin,
}

impl Screen {
    pub fn new() -> Result<Screen> {
        let skin = Skin::new();
        let board_area = Area::new(0, 0, 10, 10);
        let mut screen = Screen {
            width: 0,
            height: 0,
            board_area,
            skin,
        };
        screen.read_size()?;
        Ok(screen)
    }
    pub fn set_terminal_size(&mut self, w: u16, h: u16) {
        if w < 8 || h < 10 {
            return; // I'm just giving up
        }
        self.width = w;
        self.height = h;
        self.board_area.left = 0;
        self.board_area.top = 0;
        self.board_area.width = w;
        self.board_area.height = h - 3; // should crash on small screens
    }
    pub fn read_size(&mut self) -> Result<()> {
        let (w, h) = termimad::terminal_size();
        self.set_terminal_size(w, h);
        Ok(())
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
