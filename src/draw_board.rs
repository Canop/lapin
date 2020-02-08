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
}
impl<'d> BoardDrawer<'d> {
    pub fn new(
        board: &'d Board,
        w: &'d mut W,
        screen: &'d Screen,
    ) -> Self {
        Self { board, w, screen }
    }

    pub fn to_screen(&self, pos: Pos) -> Option<ScreenPos> {
        pos.to_screen(self.board.lapin.pos, &self.screen.board_area)
    }

    fn draw_chr(
        &mut self,
        pos: Pos,
        chr: char,
        color: Color,
    ) -> Result<()> {
        if let Some(sp) = pos.to_screen(self.board.lapin.pos, &self.screen.board_area) {
            debug!("sp: {:?}", sp);
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
        debug!("area_x={:?} area_y={:?}", area_x, area_y);

        // drawing the background
        let mut last_cell = VOID;
        self.w.queue(self.screen.skin.bg_command(last_cell))?;
        let mut pos = Pos{x: 0, y: area_y};
        for j in 0..self.screen.board_area.height {
            let sy = self.screen.board_area.top + j;
            pos.y += 1;
            let sx = self.screen.board_area.left;
            pos.x = area_x;
            self.w.queue(cursor::MoveTo(sx, sy))?;
            for _ in 0..self.screen.board_area.width {
                pos.x += 1;
                let cell = self.board.get(pos);
                if cell != last_cell {
                    self.w.queue(self.screen.skin.bg_command(cell))?;
                    last_cell = cell;
                }
                self.w.queue(Print(' '))?;
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

