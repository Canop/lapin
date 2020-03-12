use {
    anyhow::Result,
    crate::{
        app::{
            Context,
            State,
            StateTransition,
        },
        campaign::LoadedCampaign,
        display::{
            mad_skin,
            Screen,
            Status,
        },
        persist::Level,
    },
    crossterm::{
        event::{
            KeyCode,
            KeyEvent,
        },
    },
    std::fmt::Write,
    super::*,
    termimad::{
        Event,
        TextView,
    },
};

/// A screen letting the user choose a level.
///
/// The current implementation is based on a raw text_view.
/// A better one would (will?) be based on another termimad view,
/// maybe a list_view or a new one.
pub struct ChooseLevelState {
    status: Status,
    loaded_campaign: LoadedCampaign,
    selection: usize, // index of the selected level
    nb_playable_levels: usize,
    scroll: usize,
    area_height: usize,
}

impl ChooseLevelState {
    pub fn new(
        loaded_campaign: LoadedCampaign,
    ) -> Result<Self> {
        let status = Status::from_message(
            "Hit *↓* and *↑* to change the selection, *enter* to open it, *q* to quit".to_string()
        );
        let nb_playable_levels = 0; // will be updated on check wins
        let area_height = 0; // will be updated on first draw
        Ok(Self {
            status,
            loaded_campaign,
            selection: 0,
            nb_playable_levels,
            scroll: 0,
            area_height,
        })
    }

    fn markdown(&self) -> Result<String> {
        let mut md = String::new();
        write!(md, "\n# {}\n", self.loaded_campaign.name())?;
        for (i, level) in self.loaded_campaign.levels.iter().enumerate() {
            write!(
                md,
                "\n{}{}{}",
                if i >= self.nb_playable_levels {
                    "#### " // unplayable level
                } else if i == self.selection {
                    "### "  // selected level
                } else {
                    ""
                },
                level.level.name,
                if level.won { " ` WON `" } else { "" },
            )?;
        }
        Ok(md)
    }

    fn handle_key_event(
        &mut self,
        code: KeyCode,
    ) -> Result<Option<StateTransition>> {
        Ok(match code {
            KeyCode::Up if self.selection > 0 => {
                self.selection -= 1;
                if self.scroll > 0 && self.selection < self.scroll + 3 {
                    self.scroll -= 1;
                }
                None
            }
            KeyCode::Down if self.selection < self.nb_playable_levels - 1 => {
                self.selection += 1;
                if self.selection + 8 >= self.scroll + self.area_height {
                    self.scroll += 1;
                }
                None
            }
            KeyCode::Enter => Some(StateTransition::PlayLevel {
                level_idx: self.selection,
            }),
            KeyCode::Char('?') => Some(StateTransition::Help),
            KeyCode::Char('q') => Some(StateTransition::Quit),
            _ => None,
        })
    }

    fn write_status(
        &mut self,
        con: &mut Context,
        screen: &Screen,
    ) -> Result<()> {
        self.status.display(con, screen)
    }

    /// update the won property of levels and determine
    /// what levels are open to the player
    fn check_wins(
        &mut self,
    ) -> Result<()> {
        self.loaded_campaign.check_wins()?;
        self.nb_playable_levels = if self.loaded_campaign.campaign.allow_all_levels {
            self.loaded_campaign.campaign.levels.len()
        } else {
            let mut nb = 0;
            for level in &self.loaded_campaign.levels {
                nb += 1;
                if !level.won {
                    break;
                }
            }
            nb
        };
        debug!("nb_playable_levels: {:?}", self.nb_playable_levels);
        Ok(())
    }

}

impl State for ChooseLevelState {

    fn label(&self) -> &'static str {
        "level choice"
    }

    fn run(
        &mut self,
        con: &mut Context,
    ) -> Result<StateTransition> {
        let mut screen = Screen::new(LAYOUT);
        let skin = mad_skin::make(&con.skin);
        // we do this here so that wins are verified when coming back from a game
        self.check_wins()?;
        loop {
            self.write_status(con, &screen)?;
            let md = self.markdown()?;
            self.area_height = screen.areas.board.height as usize;
            let text = skin.area_text(&md, &screen.areas.board);
            let mut text_view = TextView::from(
                &screen.areas.board,
                &text,
            );
            text_view.set_scroll(self.scroll as i32);
            text_view.write_on(con.w)?;
            let event = con.dam.next_event().unwrap();
            con.dam.unblock();
            match event {
                Event::Key(KeyEvent { code, .. }) => {
                    let next_state = self.handle_key_event(code)?;
                    if let Some(next_state) = next_state {
                        return Ok(next_state);
                    }
                }
                Event::Resize(width, height) => {
                    screen.set_terminal_size(width, height);
                }
                _ => {
                    debug!("ignored event: {:?}", event);
                }
            }
        }
    }

    fn get_level(
        &self,
        level_idx: usize,
    ) -> Option<Level> {
        self.loaded_campaign.levels
            .get(level_idx)
            .map(|ll| ll.level.clone())
    }

}

