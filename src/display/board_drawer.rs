use {
    crate::{
        core::*,
        pos::*,
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
    super::*,
    termimad::{
        StyledChar,
    },
};

/// a struct able to do drawing and animations, centered
/// around an arbitrary point (the lapin position if
/// created with `new`). A new board_drawer must be rebuilt
/// in case of board change or screen resize
pub struct BoardDrawer<'d> {
    pub board: &'d Board,
    pub w: &'d mut W,
    pub screen: &'d Screen,
    pub pos_converter: PosConverter,
    pub actor_map: ActorPosMap,
}
impl<'d> BoardDrawer<'d> {
    pub fn new(
        board: &'d Board,
        w: &'d mut W,
        screen: &'d Screen,
    ) -> Self {
        Self::new_around(board, w, screen, board.lapin_pos())
    }

    pub fn new_around(
        board: &'d Board,
        w: &'d mut W,
        screen: &'d Screen,
        center: Pos,
    ) -> Self {
        let pos_converter = PosConverter::from(center, screen);
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
        let terrain = self.board.get(pos);
        self.draw_chr_bg(pos, chr, color, terrain.bg(&self.screen.skin))
    }

    pub fn draw_fg(
        &mut self,
        pos: Pos,
        sc: &StyledChar,
    ) -> Result<()> {
        if let Some(c) = sc.get_fg() {
            self.draw_chr(pos, sc.get_char(), c)?;
        }
        Ok(())
    }

    pub fn draw(
        &mut self,
    ) -> Result<()> {

        // background and items
        let mut last_terrain = Terrain::Mud;
        self.w.queue(last_terrain.bg_command(&self.screen.skin))?;
        for j in 0..self.screen.areas.board.height {
            let sy = self.screen.areas.board.top + j;
            let mut sx = self.screen.areas.board.left;
            self.w.queue(cursor::MoveTo(sx, sy))?;
            for _ in 0..self.screen.areas.board.width {
                let pos = self.pos_converter.to_real(ScreenPos::new(sx, sy));
                let terrain = self.board.get(pos);
                if terrain != last_terrain {
                    self.w.queue(terrain.bg_command(&self.screen.skin))?;
                    last_terrain = terrain;
                }
                if let Some(actor) = self.actor_map.get(pos) {
                    actor.skin(&self.screen.skin).queue(self.w)?;
                    self.w.queue(terrain.bg_command(&self.screen.skin))?;
                } else if let Some(item) = self.board.items.get(pos) {
                    item.kind.skin(&self.screen.skin).queue(self.w)?;
                    self.w.queue(terrain.bg_command(&self.screen.skin))?;
                } else {
                    self.w.queue(Print(' '))?;
                }
                sx += 1;
            }
        }

        Ok(())
    }

}

