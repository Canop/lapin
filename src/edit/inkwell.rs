
use {
    anyhow::Result,
    crate::{
        app::Context,
        display::Screen,
        pos::ScreenPos,
        core::Terrain,
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

/// draw ink_well in the cursor position
fn draw_inkwell(
    con: &mut Context,
    ink: Ink,
    selected: bool,
) -> Result<()> {
    let skin = &con.skin;
    let cs = &skin.editor.paragraph.compound_style;
    match ink {
        Ink::EraseTerrain => {
            cs.queue(con.w, ERASER_CHAR)?;
        }
        Ink::Terrain(terrain) => {
            con.w.queue(terrain.bg_command(skin))?;
            write!(con.w, " ")?;
        }
        Ink::EraseItem => {
            cs.queue(con.w, ERASER_CHAR)?;
        }
        Ink::Item(item_kind) => {
            let mut item_skin = item_kind.skin(&skin).clone();
            item_skin.set_bg(Terrain::Mud.bg(skin));
            //if let Some(c) = skin.editor.paragraph.compound_style.get_bg() {
            //    item_skin.set_bg(c);
            //}
            item_skin.queue(con.w)?;
        }
        Ink::EraseActor => {
            cs.queue(con.w, ERASER_CHAR)?;
        }
        Ink::Actor(actor_kind) => {
            let mut actor_skin = actor_kind.skin(&skin).clone();
            if let Some(c) = skin.editor.paragraph.compound_style.get_bg() {
                actor_skin.set_bg(c);
            }
            actor_skin.queue(con.w)?;
        }
    }
    if selected {
        con.w.queue(cursor::MoveDown(1))?;
        con.w.queue(cursor::MoveLeft(1))?;
        let mark = format!("▴ {}", ink);
        let len = mark.len() as u16;
        con.skin.editor.paragraph.compound_style.queue(con.w, mark)?;
        con.w.queue(cursor::MoveLeft(len - 3))?;
        con.w.queue(cursor::MoveUp(1))?;
    }
    Ok(())
}

/// draw all the inks and return a vector of the inkwells with
/// the correct positions.
///
/// The cursor must already be at sp: this function doesn't do
/// the goto. sp is modified to progress with the drawing.
pub fn draw_inkwells(
    con: &mut Context,
    screen: &Screen,
    sp: &mut ScreenPos,
    label: &str,
    inks: &[Ink],
    selected_ink: Ink,
) -> Result<Vec<InkWell>> {
    let mut inkwells = Vec::new();
    let label_len = label.len();
    let cs = con.skin.editor.paragraph.compound_style.clone();
    cs.queue_str(con.w, label)?;
    sp.x += label_len as u16;
    for &ink in inks {
        if screen.areas.ink_margin == 1 {
            cs.queue(con.w, ' ')?;
            sp.x += 1;
        }
        draw_inkwell(
            con,
            ink,
            selected_ink == ink,
        )?;
        inkwells.push(InkWell { sp:*sp, ink });
        sp.x += 1;
    }
    Ok(inkwells)
}

