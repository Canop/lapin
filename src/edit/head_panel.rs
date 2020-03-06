
use {
    anyhow::Result,
    crate::{
        core::Board,
        display::{
            Screen,
            W,
        },
        pos::ScreenPos,
    },
    crossterm::{
        event::KeyCode,
        terminal::ClearType,
    },
    super::{
        drawing_action::*,
        ink::*,
        inkwell::*,
    },
    termimad::{
        Area,
        InputField,
    },
};

/// display widgets to edit general level stuff:
/// - name
/// - default terrain
pub struct EditorHeadPanel {
    inkwells: Vec<InkWell>,
    name_area: Area,
    name_field: Option<InputField>,
}

impl EditorHeadPanel {

    pub fn new() -> Self {
        let inkwells = Vec::new(); // this will be filled when drawing
        let name_area = Area::uninitialized();
        Self {
            inkwells,
            name_area,
            name_field: None,
        }
    }

    pub fn draw(
        &mut self,
        w: &mut W,
        board: & Board,
        screen: & Screen,
    ) -> Result<()> {
        let area = &screen.areas.header;
        let cs = &screen.skin.editor.paragraph.compound_style;
        self.inkwells.clear();

        // clear first line
        screen.goto(w, 0, area.top)?;
        cs.clear(w, ClearType::UntilNewLine)?;

        // clear line below inkwells because we'll draw the marks
        screen.goto(w, 0, area.top + 2)?;
        cs.clear(w, ClearType::UntilNewLine)?;

        // default Terrain
        let mut sp = ScreenPos::new(0, area.top + 1);
        sp.goto(w)?;
        self.inkwells.extend(draw_inkwells(
            w,
            screen,
            &mut sp,
            " Default terrain:",
            &TERRAIN_INKS[1..],
            Ink::Terrain(board.default_terrain()),
        )?);

        cs.clear(w, ClearType::UntilNewLine)?;

        // name
        sp.x += 5;
        sp.goto(w)?;
        self.name_area = Area::new(
            sp.x,
            screen.areas.header.top + 1,
            area.width - sp.x - 2,
            1,
        );

        if let Some(name_field) = &self.name_field {
            name_field.display_on(w)?;
        } else {
            cs.queue_str(w, if board.name.is_empty() {
                "-unnamed level-"
            } else {
                &board.name
            })?;
        }

        Ok(())
    }

    /// return true when the call effectively closed the field
    /// (ie it was open)
    pub fn close_name_field(
        &mut self,
        board: &mut Board,
    ) -> bool {
        if let Some(name_field) = self.name_field.take() {
            board.name = name_field.get_content();
            true
        } else {
            false
        }
    }

    /// tell the panel there was a click outside of it, which
    /// means the name field should be closed
    pub fn click_outside(
        &mut self,
        board: &mut Board,
    ) {
        self.close_name_field(board);
    }

    /// return true if the event was handled
    /// (which is normally the case when the input field is shown)
    pub fn handle_key_event(
        &mut self,
        code: KeyCode,
        board: &mut Board,
    ) -> bool {
        if let Some(name_field) = &mut self.name_field {
            if name_field.apply_keycode_event(code) {
                true
            } else {
                match code {
                    KeyCode::Esc | KeyCode::Enter => {
                        self.close_name_field(board);
                        true
                    }
                    _ => false
                }
            }
        } else {
            false
        }
    }

    pub fn click(
        &mut self,
        sp: ScreenPos,
        board: &mut Board,
    ) -> Option<DrawingAction> {
        if sp.is_in(&self.name_area) {
            if let Some(name_field) = &mut self.name_field {
                name_field.apply_click_event(sp.x, sp.y);
            } else {
                let mut field = InputField::new(self.name_area.clone());
                field.set_content(&board.name);
                self.name_field = Some(field);
            }
        } else {
            self.close_name_field(board);
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
        }
        None
    }
}
