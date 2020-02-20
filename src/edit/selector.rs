
use {
    anyhow::Result,
    crate::{
        actor::*,
        consts::*,
        io::W,
        item::*,
        pos::ScreenPos,
        screen::*,
    },
    crossterm::{
        cursor,
        style::{
            SetForegroundColor,
        },
        terminal::ClearType,
        QueueableCommand,
    },
    minimad::{
        Alignment,
        Composite,
    },
    std::io::Write,
    super::{
        pen::*,
    },
};

/// a place on the screen on which you click to
/// select an ink
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct InkWell {
    sp: ScreenPos,
    ink: PenInk,
}

pub struct SelectorPanel<'s> {
    w: &'s mut W,
    pen: &'s mut Pen,
    screen: &'s Screen,
    inkwells: Vec<InkWell>,
}

const INK_MARGIN: u16 = 1; // 0 or 1

impl<'s> SelectorPanel<'s> {
    pub fn new(
        w: &'s mut W,
        pen: &'s mut Pen,
        screen: &'s Screen,
    ) -> Self {
        let inkwells = Vec::new(); // this will be filled when drawing
        Self {
            w,
            pen,
            screen,
            inkwells,
        }
    }

    fn draw_eraser(&mut self) -> Result<()> {
        self.screen.skin.editor.paragraph.compound_style.queue(self.w, '╳')?;
        Ok(())
    }

    fn clear_line(&mut self) -> Result<()> {
        self.screen.skin.editor.paragraph.compound_style.clear(self.w, ClearType::UntilNewLine)?;
        Ok(())
    }

    // draw ink_well in the cursor position (which is assumed
    // to be sp) and store the inkwell so that it is used on click
    fn draw_inkwell(&mut self, sp: ScreenPos, ink: PenInk) -> Result<()> {
        let skin = &self.screen.skin;
        match ink {
            PenInk::EraseTerrain => {
                self.draw_eraser()?;
            }
            PenInk::Terrain(cell) => {
                self.w.queue(skin.bg_command(cell))?;
                write!(self.w, " ")?;
            }
            PenInk::EraseItem => {
                self.draw_eraser()?;
            }
            PenInk::Item(item_kind) => {
                let item_skin = item_kind.skin(&skin);
                self.w.queue(item_skin.styled_char_over(Some(skin.bg(FIELD))))?;
            }
            PenInk::EraseActor => {
                self.draw_eraser()?;
            }
            PenInk::Actor(actor_kind) => {
                let actor_skin = actor_kind.skin(&skin);
                self.w.queue(actor_skin.styled_char_over(
                    skin.editor.paragraph.compound_style.get_bg()
                ))?;
            }
        }
        if ink == self.pen.ink {
            self.mark_below_and_back()?;
        }
        self.inkwells.push(InkWell { sp, ink });
        Ok(())
    }

    pub fn draw(&mut self) -> Result<()> {
        let area = &self.screen.areas.selector;
        let skin = &self.screen.skin;
        self.inkwells.clear();

        // clear first line
        self.screen.goto(self.w, 0, area.top)?;
        self.clear_line()?;

        // clear line below terrains because we'll draw the marks
        self.screen.goto(self.w, 0, area.top + 2)?;
        self.clear_line()?;

        // Terrains
        let mut sp = ScreenPos::new(0, area.top + 1);
        sp.goto(self.w)?;
        let label_len = 12;
        skin.editor.write_composite_fill(
            self.w,
            Composite::from_inline("Terrain: "),
            label_len,
            Alignment::Right,
        )?;
        sp.x += label_len as u16;
        self.draw_inkwell(sp, PenInk::EraseTerrain)?;
        sp.x += 1;
        for i in 0..NB_TERRAINS {
            if INK_MARGIN == 1 {
                skin.editor.paragraph.compound_style.queue(self.w, ' ')?;
                sp.x += 1;
            }
            self.draw_inkwell(sp, PenInk::Terrain(i))?;
            sp.x += 1;
        }

        // Items
        let label_len = 10;
        skin.editor.write_composite_fill(
            self.w,
            Composite::from_inline("Items: "),
            label_len,
            Alignment::Right,
        )?;
        sp.x += label_len as u16;
        self.draw_inkwell(sp, PenInk::EraseItem)?;
        sp.x += 1;
        for &item in ITEMS {
            if INK_MARGIN == 1 {
                skin.editor.paragraph.compound_style.queue(self.w, ' ')?;
                sp.x += 1;
            }
            self.draw_inkwell(sp, PenInk::Item(item))?;
            sp.x += 1;
        }

        // Actors
        let label_len = 11;
        skin.editor.write_composite_fill(
            self.w,
            Composite::from_inline("Actors: "),
            label_len,
            Alignment::Right,
        )?;
        sp.x += label_len as u16;
        self.draw_inkwell(sp, PenInk::EraseActor)?;
        sp.x += 1;
        for &actor in ACTORS {
            if INK_MARGIN == 1 {
                skin.editor.paragraph.compound_style.queue(self.w, ' ')?;
                sp.x += 1;
            }
            self.draw_inkwell(sp, PenInk::Actor(actor))?;
            sp.x += 1;
        }

        self.clear_line()?;

        Ok(())
    }

   #[allow(dead_code)]
    pub fn circle_around(&mut self, x: u16, y: u16) -> Result<()> {
        self.screen.goto(self.w, x-1, y-1)?;
        self.screen.skin.editor.paragraph.compound_style.queue_bg(self.w)?;
        self.w.queue(SetForegroundColor(self.screen.skin.editor_circle))?;
        write!(self.w, "▗")?;
        write!(self.w, "▄")?;
        write!(self.w, "▖")?;
        self.screen.goto(self.w, x-1, y)?;
        write!(self.w, "▐")?;
        self.screen.goto(self.w, x+1, y)?;
        write!(self.w, "▌")?;
        self.screen.goto(self.w, x-1, y+1)?;
        write!(self.w, "▝")?;
        write!(self.w, "▀")?;
        write!(self.w, "▘")?;
        Ok(())
    }

   #[allow(dead_code)]
    pub fn mark_below(&mut self, x: u16, y: u16) -> Result<()> {
        self.screen.goto(self.w, x, y+1)?;
        self.screen.skin.editor.paragraph.compound_style.queue(self.w, '▴')?;
        Ok(())
    }

    pub fn mark_below_and_back(&mut self) -> Result<()> {
        self.w.queue(cursor::MoveDown(1))?;
        self.w.queue(cursor::MoveLeft(1))?;
        self.screen.skin.editor.paragraph.compound_style.queue(self.w, '▴')?;
        self.w.queue(cursor::MoveUp(1))?;
        Ok(())
    }

    pub fn click(&mut self, sp: ScreenPos) {
        debug!("selector click {:?}", sp);
        for inkwell in &self.inkwells {
            if inkwell.sp == sp {
                self.pen.set_ink(inkwell.ink);
                return;
            }
        }
    }

}
