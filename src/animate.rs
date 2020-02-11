use {
    anyhow::Result,
    crate::{
        draw_board::*,
        pos::*,
        task_sync::*,
        world::*,
    },
    crossterm::{
        cursor,
        style::{
            Attributes,
            Color,
            ContentStyle,
            PrintStyledContent,
        },
        QueueableCommand,
    },
    std::{
        thread,
        time::Duration,
    },
};

static HORIZONTAL_BC: [char; 9] = [' ', '▏', '▎', '▍', '▌', '▋', '▊', '▉', '█'];
static VERTICAL_BC:   [char; 9] = [' ', '▁', '▂', '▃', '▄', '▅', '▆', '▇', '█'];

impl<'d> BoardDrawer<'d> {
    fn draw_bicolor_horizontal(
        &mut self,
        sp: Option<ScreenPos>,
        left_color: Color,
        right_color: Color,
        av: usize, // part of the left color, in [0, 8] (0 is 100% right_color)
    ) -> Result<()> {
        if let Some(sp) = sp {
            let cs = ContentStyle {
                foreground_color: Some(left_color),
                background_color: Some(right_color),
                attributes: Attributes::default(),
            };
            self.w.queue(cursor::MoveTo(sp.x, sp.y))?;
            self.w.queue(PrintStyledContent(cs.apply(HORIZONTAL_BC[av])))?;
        }
        Ok(())
    }
    fn draw_bicolor_vertical(
        &mut self,
        sp: Option<ScreenPos>,
        top_color: Color,
        bottom_color: Color,
        av: usize, // part of the left color, in [0, 8] (0 is 100% right_color)
    ) -> Result<()> {
        if let Some(sp) = sp {
            let cs = ContentStyle {
                foreground_color: Some(top_color),
                background_color: Some(bottom_color),
                attributes: Attributes::default(),
            };
            self.w.queue(cursor::MoveTo(sp.x, sp.y))?;
            self.w.queue(PrintStyledContent(cs.apply(VERTICAL_BC[av])))?;
        }
        Ok(())
    }
    fn draw_move_step(
        &mut self,
        start: Pos,
        dir: Dir,
        color: Color,
        killed_id: Option<usize>,
        av: usize, // in [0, 8]
    ) -> Result<()> {
        let sp_start = self.to_screen(start);
        let dst = start.in_dir(dir);
        let sp_dst = self.to_screen(dst);
        let start_bg = self.screen.skin.bg(self.board.get(start));
        let dst_bg = self.screen.skin.bg(self.board.get(dst));
        match dir {
            Dir::Up => {
                self.draw_bicolor_vertical(sp_start, start_bg, color, av)?;
                self.draw_bicolor_vertical(sp_dst, color, dst_bg, av)?;
            }
            Dir::Left => {
                self.draw_bicolor_horizontal(sp_start, color, start_bg, 8-av)?;
                self.draw_bicolor_horizontal(sp_dst, dst_bg, color, 8-av)?;
            }
            Dir::Down => {
                self.draw_bicolor_vertical(sp_start, color, start_bg, 8-av)?;
                self.draw_bicolor_vertical(sp_dst, dst_bg, color, 8-av)?;
            }
            Dir::Right => {
                self.draw_bicolor_horizontal(sp_start, start_bg, color, av)?;
                self.draw_bicolor_horizontal(sp_dst, color, dst_bg, av)?;
            }
            _ => {
                // for diagonals, for now, we just alternate between one
                // and the other. This is about OK because diagonals are for
                // kills
                if av%2==1 {
                    self.draw_chr(start, '█', color)?;
                    if let Some(kind) = killed_id.map(|id| self.board.actors[id].kind) {
                        self.draw_chr(dst, kind.skin(&self.screen.skin).chr, Color::Red)?;
                    }
                } else {
                    self.draw_chr(start, ' ', color)?;
                    self.draw_chr(dst, '█', color)?;
                }
            }
        }
        Ok(())
    }
    /// the board is presumed already drawn on the state pre-move,
    /// we draw only the moving things
    pub fn animate(
        &mut self,
        world_move: &WorldMove,
        dam: &mut Dam,
    ) -> Result<()> {
        for av in 0..=8 {
            for actor_move in &world_move.actor_moves {
                let actor_id = actor_move.actor_id;
                let actor = self.board.actors[actor_id];
                self.draw_move_step(
                    actor.pos,
                    actor_move.dir,
                    actor.kind.skin(&self.screen.skin).color,
                    actor_move.target_id,
                    av,
                )?;
            }
            if !dam.try_wait(Duration::from_millis(40)) {
                debug!("break animation");
                break;
            }
        }
        Ok(())
    }
}

