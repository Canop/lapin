
/// The help modal view is displayed over state runners
/// with their collaboration
///

use {
    anyhow::Result,
    crate::{
        app_state::StateTransition,
        io::W,
        layout::Layout,
        pos::*,
        screen::Screen,
        skin::Skin,
        status::Status,
        task_sync::*,
        terrain::Terrain,
    },
    crossterm::{
        event::{
            KeyCode,
            KeyEvent,
        },
        style::Color::*,
    },
    minimad::{
        Compound,
    },
    termimad::{
        Alignment,
        Event,
        MadSkin,
        MadView,
    },
};

fn make_mad_skin(skin: &Skin) -> MadSkin {
    let mut ms = MadSkin::default();
    let bg = Terrain::Mud.bg(skin);
    ms.set_headers_fg(AnsiValue(178));
    ms.paragraph.align = Alignment::Center;
    let mut lapin = skin.lapin.clone();
    lapin.set_bg(bg);
    let mut carrot = skin.carrot.clone();
    carrot.set_bg(bg);
    let mut wine = skin.wine.clone();
    wine.set_bg(bg);
    ms.special_chars.insert(Compound::raw_str("H").code(), skin.hunter.clone());
    ms.special_chars.insert(Compound::raw_str("K").code(), skin.knight.clone());
    ms.special_chars.insert(Compound::raw_str("L").code(), lapin);
    ms.special_chars.insert(Compound::raw_str("F").code(), skin.fox.clone());
    ms.special_chars.insert(Compound::raw_str("S").code(), skin.sheep.clone());
    ms.special_chars.insert(Compound::raw_str("W").code(), skin.wolf.clone());
    ms.special_chars.insert(Compound::raw_str("g").code(), Terrain::Grass.bg_as_styled_char(skin));
    ms.special_chars.insert(Compound::raw_str("s").code(), Terrain::Sand.bg_as_styled_char(skin));
    ms.special_chars.insert(Compound::raw_str("c").code(), carrot);
    ms.special_chars.insert(Compound::raw_str("w").code(), wine);
    ms.italic.set_fg(AnsiValue(178));
    ms.scrollbar.thumb.set_fg(AnsiValue(178));
    ms.set_global_bg(bg);
    ms
}

pub struct View {
    layout: Layout, // whole screen layout
    markdown: &'static str,
}

impl View {

    pub fn new(markdown: &'static str, layout: Layout) -> Self {
        Self {
            markdown,
            layout,
        }
    }

    pub fn run(
        &mut self,
        w: &mut W,
        dam: &mut Dam,
    ) -> Result<Option<StateTransition>> {
        let mut screen = Screen::new(self.layout);
        let skin = make_mad_skin(&screen.skin);
        let mut mad_view = MadView::from(
            self.markdown.to_string(),
            screen.areas.help.clone(),
            skin,
        );
        loop {
            Status::from_message("Hit *esc* to close the help").display(w, &screen)?;
            mad_view.write_on(w)?;
            let event = dam.next_event().unwrap();
            dam.unblock();
            debug!("help event: {:?}", event);
            match event {
                Event::Key(KeyEvent { code, .. }) => {
                    match code {
                        KeyCode::Up => {
                            mad_view.try_scroll_lines(-1);
                        }
                        KeyCode::Down => {
                            mad_view.try_scroll_lines(1);
                        }
                        KeyCode::PageUp => {
                            mad_view.try_scroll_pages(-1);
                        }
                        KeyCode::PageDown => {
                            mad_view.try_scroll_pages(1);
                        }
                        KeyCode::Char('q') => {
                            return Ok(Some(StateTransition::Quit));
                        }
                        KeyCode::Esc => {
                            return Ok(None);
                        }
                        _ => {
                            debug!("ignored code");
                        }
                    }
                }
                Event::Resize(width, height) => {
                    screen.set_terminal_size(width, height);
                    mad_view.resize(&screen.areas.help);
                }
                Event::Click(x, y, ..) => {
                    let sp = ScreenPos{ x, y };
                    debug!("click in sp={:?}", sp);
                }
                _ => {
                    debug!("ignored event: {:?}", event);
                }
            }
        }
    }
}
