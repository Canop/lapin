use {
    anyhow::Result,
    crate::{
        app::Dam,
        core::*,
        pos::*,
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
        time::Duration,
    },
    super::*,
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
        av: usize, // in [0, 8]
    ) -> Result<()> {
        let sp_start = self.pos_converter.to_screen(start);
        let dst = start.in_dir(dir);
        let sp_dst = self.pos_converter.to_screen(dst);
        let start_bg = self.board.get(start).bg(&self.screen.skin);
        let dst_bg = if let Some(dst_actor) = self.actor_map.get(dst) {
            dst_actor.kind.skin(&self.screen.skin).get_fg().unwrap()
        } else {
            self.board.get(dst).bg(&self.screen.skin)
        };
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
                // should not happen
            }
        }
        Ok(())
    }
    fn draw_kill_step(
        &mut self,
        start: Pos,
        dir: Dir,
        color: Color,
        killed_id: Option<usize>,
        av: usize, // in [0, 8]
    ) -> Result<()> {
        let dst = start.in_dir(dir);
        if av%2==1 {
            self.draw_chr(start, '█', color)?;
            if let Some(kind) = killed_id.map(|id| self.board.actors[id].kind) {
                self.draw_chr(dst, kind.skin(&self.screen.skin).get_char(), Color::Red)?;
            }
        } else {
            self.draw_chr(start, ' ', color)?;
            self.draw_chr(dst, '█', color)?;
        }
        Ok(())
    }
    fn draw_fire_step(
        &mut self,
        start: Pos,
        dir: Dir,
        target_id: Option<usize>,
        av: usize, // in [0, 8]
    ) -> Result<()> {
        let mut pos = start;
        for _ in av/3..av {
            pos = pos.in_dir(dir);
            if let Some(target_id) = target_id {
                if self.board.actors[target_id].pos == pos {
                    break;
                }
            }
            let fg_skin = match dir {
                Dir::Up | Dir::Down => &self.screen.skin.fire_vertical,
                _ => &self.screen.skin.fire_horizontal,
            };
            self.draw_fg(pos, fg_skin)?;
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
                let mut actor = self.board.actors[actor_id];
                match actor_move.action {
                    Action::Moves(dir) => {
                        self.draw_move_step(
                            actor.pos,
                            dir,
                            actor.kind.skin(&self.screen.skin).get_fg().unwrap(),
                            av,
                        )?;
                    }
                    Action::Eats(dir) => {
                        self.draw_kill_step(
                            actor.pos,
                            dir,
                            actor.kind.skin(&self.screen.skin).get_fg().unwrap(),
                            actor_move.target_id,
                            av,
                        )?;
                    }
                    Action::Fires(dir) => {
                        self.draw_fire_step(
                            actor.pos,
                            dir,
                            actor_move.target_id,
                            av,
                        )?;
                    }
                    Action::Aims(dir) => {
                        actor.state.aim = Some(dir);
                        self.draw_fg(
                            actor.pos,
                            &actor.skin(&self.screen.skin),
                        )?;
                    }
                    _ => {}
                }
            }
            if !dam.try_wait(Duration::from_millis(50)) {
                break;
            }
        }
        Ok(())
    }
}

