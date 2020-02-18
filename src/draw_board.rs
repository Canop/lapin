use {
    crate::{
        actor::*,
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
    pub pos_converter: PosConverter,
    pub actor_map: ActorPosMap,
}
impl<'d> BoardDrawer<'d> {
    /// a new board_drawer must be created if the screen is resized
    /// or when a move is played
    pub fn new(
        board: &'d Board,
        w: &'d mut W,
        screen: &'d Screen,
    ) -> Self {
        let pos_converter = PosConverter::from(board.lapin_pos(), screen);
        let actor_map = board.actor_pos_map();
        Self { board, w, screen, pos_converter, actor_map }
    }

    pub fn draw_chr_bg(
        &mut self,
        pos: Pos,
        chr: char,
        fg_color: Color,
        bg_color: Color,
    ) -> Result<()> {
        if let Some(sp) = self.pos_converter.to_screen(pos) {
            let cs = ContentStyle {
                foreground_color: Some(fg_color),
                background_color: Some(bg_color),
                attributes: Attributes::default(),
            };
            self.w.queue(cursor::MoveTo(sp.x, sp.y))?;
            self.w.queue(PrintStyledContent(cs.apply(chr)))?;
        }
        Ok(())
    }

    pub fn draw_chr(
        &mut self,
        pos: Pos,
        chr: char,
        color: Color,
    ) -> Result<()> {
        let cell = self.board.get(pos);
        self.draw_chr_bg(pos, chr, color, self.screen.skin.bg(cell))
    }

    pub fn draw_fg(
        &mut self,
        pos: Pos,
        fg_skin: FgSkin,
    ) -> Result<()> {
        self.draw_chr(pos, fg_skin.chr, fg_skin.color)
    }

    pub fn draw(
        &mut self,
    ) -> Result<()> {

        // background and items
        let mut last_cell = FIELD;
        self.w.queue(self.screen.skin.bg_command(last_cell))?;
        for j in 0..self.screen.areas.board.height {
            let sy = self.screen.areas.board.top + j;
            let mut sx = self.screen.areas.board.left;
            self.w.queue(cursor::MoveTo(sx, sy))?;
            for _ in 0..self.screen.areas.board.width {
                let pos = self.pos_converter.to_real(ScreenPos::new(sx, sy));
                let cell = self.board.get(pos);
                if cell != last_cell {
                    self.w.queue(self.screen.skin.bg_command(cell))?;
                    last_cell = cell;
                }
                if let Some(actor) = self.actor_map.get(pos) {
                    let fg_skin = actor.skin(&self.screen.skin);
                    self.w.queue(fg_skin.fg_command())?;
                    self.w.queue(Print(fg_skin.chr))?;
                } else if let Some(item) = self.board.items.get(pos) {
                    self.w.queue(self.screen.skin.styled_item_char(item, cell))?;
                    self.w.queue(self.screen.skin.bg_command(cell))?;
                } else {
                    self.w.queue(Print(' '))?;
                }
                sx += 1;
            }
        }

        Ok(())
    }

}

