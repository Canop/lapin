
use {
    anyhow::Result,
    crate::{
        board::*,
        io::W,
        pos::ScreenPos,
        screen::*,
    },
    crossterm::{
        terminal::ClearType,
    },
    super::{
        drawing_action::*,
        ink::*,
        inkwell::*,
    },
};

/// display widgets to edit general level stuff:
/// - name
/// - default terrain
pub struct EditorHeadPanel<'s> {
    board: &'s Board,
    screen: &'s Screen,
    inkwells: Vec<InkWell>,
}

impl<'s> EditorHeadPanel<'s> {

    pub fn new(
        board: &'s Board,
        screen: &'s Screen,
    ) -> Self {
        let inkwells = Vec::new(); // this will be filled when drawing
        Self {
            board,
            inkwells,
            screen,
        }
    }
    pub fn draw(&mut self, w: &mut W) -> Result<()> {
        let area = &self.screen.areas.header;
        let cs = &self.screen.skin.editor.paragraph.compound_style;
        self.inkwells.clear();

        // clear first line
        self.screen.goto(w, 0, area.top)?;
        cs.clear(w, ClearType::UntilNewLine)?;

        // clear line below inkwells because we'll draw the marks
        self.screen.goto(w, 0, area.top + 2)?;
        cs.clear(w, ClearType::UntilNewLine)?;

        // Default Terrain
        let mut sp = ScreenPos::new(0, area.top + 1);
        sp.goto(w)?;
        self.inkwells.extend(draw_inkwells(
            w,
            self.screen,
            &mut sp,
            " Default terrain:",
            &TERRAIN_INKS[1..],
            Ink::Terrain(self.board.default_terrain()),
        )?);
        cs.clear(w, ClearType::UntilNewLine)?;

        Ok(())
    }

    pub fn click(&mut self, sp: ScreenPos) -> Option<DrawingAction> {
        debug!("head_panel click {:?}", sp);
        for inkwell in &self.inkwells {
            if inkwell.sp == sp {
                match inkwell.ink {
                    Ink::Terrain(terrain) => {
                        return Some(DrawingAction::DefaultTerrain(terrain));
                    }
                    _ => {
                        warn!("unexptected ink");
                    }
                }
            }
        }
        None
    }
}
