use {
    anyhow::Result,
    crate::{
        app::{
            Dam,
            State,
            StateTransition,
        },
        campaign::LoadedCampaign,
        display::{
            mad_skin,
            Screen,
            Status,
            W,
        },
        level::Level,
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

pub struct ChooseLevelState {
    status: Status,
    loaded_campaign: LoadedCampaign,
    selection: usize, // index of the selected level
}

impl ChooseLevelState {
    pub fn new(
        loaded_campaign: LoadedCampaign,
    ) -> Result<Self> {
        let status = Status::from_message(
            "Hit *↓* and *↑* to change the selection, *enter* to open it, *q* to quit".to_string()
        );
        let selection = 0;
        Ok(Self {
            status,
            loaded_campaign,
            selection,
        })
    }

    fn markdown(&self) -> Result<String> {
        let mut md = String::new();
        write!(md, "\n# {}\n", self.loaded_campaign.name())?;
        for (i, level) in self.loaded_campaign.levels.iter().enumerate() {
            write!(
                md,
                "\n{}{}{}",
                if i == self.selection { "### " } else { "" },
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
                None
            }
            KeyCode::Down if self.selection < self.loaded_campaign.levels.len() - 1 => {
                self.selection += 1;
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
        w: &mut W,
        screen: &Screen,
    ) -> Result<()> {
        self.status.display(w, screen)
    }

}

impl State for ChooseLevelState {

    fn label(&self) -> &'static str {
        "level choice"
    }

    fn run(
        &mut self,
        w: &mut W,
        dam: &mut Dam,
    ) -> Result<StateTransition> {
        let mut screen = Screen::new(LAYOUT);
        let skin = mad_skin::make(&screen.skin);
        // we do this here so that wins are verified when coming back from a game
        self.loaded_campaign.check_wins()?;
        loop {
            self.write_status(w, &screen)?;
            let md = self.markdown()?;
            let text = skin.area_text(&md, &screen.areas.board);
            let text_view = TextView::from(
                &screen.areas.board,
                &text,
            );
            text_view.write_on(w)?;
            let event = dam.next_event().unwrap();
            dam.unblock();
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
                //Event::Click(x, y, ..) => {
                //    let sp = ScreenPos{ x, y };
                //    let pos_converter = PosConverter::from(self.board.lapin_pos(), &screen);
                //    debug!("click in {:?}", pos_converter.to_real(sp));
                //}
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


