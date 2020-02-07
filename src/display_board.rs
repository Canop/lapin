use {
    crate::{
        board::Board,
        consts::*,
        io::W,
        pos::*,
        screen::Screen,
    },
    anyhow::Result,
    crossterm::{
        cursor,
        style::{
            Attribute,
            Attributes,
            Color,
            ContentStyle,
            Print,
            PrintStyledContent,
        },
        QueueableCommand,
    },
    std::io::Write,
    termimad::{Area, CompoundStyle, InputField, MadSkin},
};

impl Board {
    pub fn draw(
        &self,
        w: &mut W,
        screen: &Screen,
    ) -> Result<()> {
        //let screen_center = Pos::center_of(&screen.board_area);
        //debug!("screen_center: {:?}", screen_center);

        debug!("board: w={:?} h={:?}", self.width, self.height);
        debug!("area: {:?}", &screen.board_area);
        let lapin_pos = self.lapin.pos;
        debug!("lapin_pos: {:?}", lapin_pos);
        // (area_x, area_y) is the top left corner of the area in real coordinates
        let area_x = lapin_pos.x - (screen.board_area.width as Int) / 2;
        let area_y = lapin_pos.y - (screen.board_area.height as Int) / 2;
        debug!("area_x={:?} area_y={:?}", area_x, area_y);

        // drawing the background
        let mut last_cell = VOID;
        w.queue(screen.skin.bg_command(last_cell))?;
        let mut pos = Pos{x: 0, y: area_y};
        for j in 0..screen.board_area.height {
            let sy = screen.board_area.top + j;
            pos.y += 1;
            let sx = screen.board_area.left;
            pos.x = area_x;
            screen.goto(w, sx, sy)?;
            for i in 0..screen.board_area.width {
                pos.x += 1;
                let cell = self.get(pos);
                if cell != last_cell {
                    w.queue(screen.skin.bg_command(cell))?;
                    last_cell = cell;
                }
                w.queue(Print(' '))?;
            }
        }

        // le lapin!
        if let Some(sp) = lapin_pos.to_screen(lapin_pos, &screen.board_area) {
            debug!("sp: {:?}", sp);
            let cell = self.get(lapin_pos);
            let cs = ContentStyle {
                foreground_color: Some(screen.skin.lapin_fg),
                background_color: Some(screen.skin.bg(cell)),
                attributes: Attributes::default(),
            };
            screen.goto(w, sp.x, sp.y)?;
            w.queue(PrintStyledContent(cs.apply('•')));
        }

        Ok(())
    }
}
