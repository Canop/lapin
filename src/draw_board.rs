use {
    crate::{
        board::Board,
        consts::*,
        io::W,
        pos::*,
        screen::Screen,
        skin::*,
    },
    anyhow::Result,
    crossterm::{
        cursor,
        style::{
            Attributes,
            ContentStyle,
            Color,
            Print,
            PrintStyledContent,
        },
        QueueableCommand,
    },
};

pub struct BoardDrawer<'d> {
    pub board: &'d Board,
    pub w: &'d mut W,
    pub screen: &'d Screen,
    dec: Pos,
    dim: Pos,
}
impl<'d> BoardDrawer<'d> {
    /// a new board_drawer must be created if the screen is resized
    /// or when the Lapin moves
    pub fn new(
        board: &'d Board,
        w: &'d mut W,
        screen: &'d Screen,
    ) -> Self {
        let dim = Pos {
            x: screen.board_area.width as Int,
            y: screen.board_area.height as Int,
        };
        let dec = Pos {
            x: dim.x / 2 - board.lapin.pos.x,
            y: dim.y / 2 - board.lapin.pos.y,
        };
        Self { board, w, screen, dec, dim }
    }

    pub fn to_screen(&self, pos: Pos) -> Option<ScreenPos> {
        //pos.to_screen(self.board.lapin.pos, &self.screen.board_area)
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

    fn draw_chr(
        &mut self,
        pos: Pos,
        chr: char,
        color: Color,
    ) -> Result<()> {
        if let Some(sp) = self.to_screen(pos) {
            let cell = self.board.get(pos);
            let cs = ContentStyle {
                foreground_color: Some(color),
                background_color: Some(self.screen.skin.bg(cell)),
                attributes: Attributes::default(),
            };
            self.w.queue(cursor::MoveTo(sp.x, sp.y))?;
            self.w.queue(PrintStyledContent(cs.apply(chr)))?;
        }
        Ok(())
    }

    fn draw_fg(
        &mut self,
        pos: Pos,
        fg_skin: &FgSkin,
    ) -> Result<()> {
        self.draw_chr(pos, fg_skin.chr, fg_skin.color)
    }

    pub fn draw(
        &mut self,
    ) -> Result<()> {
        let lapin_pos = self.board.lapin.pos;
        // (area_x, area_y) is the top left corner of the area in real coordinates
        let area_x = lapin_pos.x - (self.screen.board_area.width as Int) / 2;
        let area_y = lapin_pos.y - (self.screen.board_area.height as Int) / 2;

        let pt = Pos::new(7, 11);
        let spt = self.to_screen(pt).unwrap();
        assert_eq!(self.to_real(spt), pt);

        // drawing the background
        let mut last_cell = VOID;
        self.w.queue(self.screen.skin.bg_command(last_cell))?;
        for j in 0..self.screen.board_area.height {
            let sy = self.screen.board_area.top + j;
            let mut sx = self.screen.board_area.left;
            self.w.queue(cursor::MoveTo(sx, sy))?;
            for _ in 0..self.screen.board_area.width {
                let pos = self.to_real(ScreenPos::new(sx, sy));
                let cell = self.board.get(pos);
                if cell != last_cell {
                    self.w.queue(self.screen.skin.bg_command(cell))?;
                    last_cell = cell;
                }
                self.w.queue(Print(' '))?;
                sx += 1;
            }
        }

        // le lapin!
        self.draw_fg(lapin_pos, &self.screen.skin.lapin)?;

        // foxes
        for fox in &self.board.foxes {
            self.draw_fg(fox.pos, &self.screen.skin.fox)?;
        }

        Ok(())
    }

}

