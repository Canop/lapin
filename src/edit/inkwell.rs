
use {
    anyhow::Result,
    crate::{
        consts::*,
        io::W,
        pos::ScreenPos,
        screen::*,
    },
    crossterm::{
        cursor,
        QueueableCommand,
    },
    std::io::Write,
    super::{
        ink::*,
    },
};

/// a place on the screen on which you click to
/// select an ink
#[derive(Debug, Clone, Copy)]
pub struct InkWell {
    pub sp: ScreenPos,
    pub ink: Ink,
}

const ERASER_CHAR: char = '╳';

// draw ink_well in the cursor position
fn draw_inkwell(
    w: &mut W,
    screen: &Screen,
    ink: Ink,
    selected: bool,
) -> Result<()> {
    let skin = &screen.skin;
    let cs = &skin.editor.paragraph.compound_style;
    match ink {
        Ink::EraseTerrain => {
            cs.queue(w, ERASER_CHAR)?;
        }
        Ink::Terrain(cell) => {
            w.queue(skin.bg_command(cell))?;
            write!(w, " ")?;
        }
        Ink::EraseItem => {
            cs.queue(w, ERASER_CHAR)?;
        }
        Ink::Item(item_kind) => {
            let mut item_skin = item_kind.skin(&skin).clone();
            item_skin.set_bg(skin.bg(FIELD));
            item_skin.queue(w)?;
        }
        Ink::EraseActor => {
            cs.queue(w, ERASER_CHAR)?;
        }
        Ink::Actor(actor_kind) => {
            let mut actor_skin = actor_kind.skin(&skin).clone();
            if let Some(c) = skin.editor.paragraph.compound_style.get_bg() {
                actor_skin.set_bg(c);
            }
            actor_skin.queue(w)?;
        }
    }
    if selected {
        w.queue(cursor::MoveDown(1))?;
        w.queue(cursor::MoveLeft(1))?;
        screen.skin.editor.paragraph.compound_style.queue(w, '▴')?;
        w.queue(cursor::MoveUp(1))?;
    }
    Ok(())
}

/// draw all the inks and return a vector of the inkwells with
/// the correct positions.
///
/// The cursor must already be at sp: this function doesn't do
/// the goto. sp is modified to progress with the drawing.
pub fn draw_inkwells(
    w: &mut W,
    screen: &Screen,
    sp: &mut ScreenPos,
    label: &str,
    inks: &[Ink],
    selected_ink: Ink,
) -> Result<Vec<InkWell>> {
    let mut inkwells = Vec::new();
    let label_len = label.len();
    let cs = &screen.skin.editor.paragraph.compound_style;
    cs.queue_str(w, label)?;
    sp.x += label_len as u16;
    for &ink in inks {
        if screen.areas.ink_margin == 1 {
            cs.queue(w, ' ')?;
            sp.x += 1;
        }
        draw_inkwell(
            w,
            screen,
            ink,
            selected_ink == ink,
        )?;
        inkwells.push(InkWell { sp:*sp, ink });
        sp.x += 1;
    }
    Ok(inkwells)
}

