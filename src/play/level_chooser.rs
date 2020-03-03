use {
    anyhow::Result,
    crate::{
        app_state::StateTransition,
        campaign::LoadedCampaign,
        io::W,
        mad_skin,
        screen::Screen,
        status::Status,
    },
    crossterm::{
        event::{
            KeyCode,
            KeyEvent,
        },
    },
    super::*,
    std::{
        fmt::Write,
    },
    termimad::{
        Event,
        TextView,
    },
};

pub struct LevelChooser {
    status: Status,
    loaded_campaign: LoadedCampaign,
    selection: usize, // index of the selected level
}

impl LevelChooser {
    pub fn new(state: PlayCampaignState) -> Result<Self> {
        let loaded_campaign = LoadedCampaign::load(
            &state.path,
            *state.bag,
        )?;
        let status = Status::from_message(
            "Hit *↓* and *↑* to change the selection, *enter* to play it, *q* to quit".to_string()
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
            if i == self.selection {
                write!(md, "\n### `K` {} `F`", level.level.name)?;
            } else {
                write!(md, "\n {}", level.level.name)?;
            }
        }
        Ok(md)
    }

    /// return the next state
    pub fn run(
        &mut self,
        w: &mut W,
        dam: &mut Dam,
    ) -> Result<StateTransition> {
        let mut screen = Screen::new(LAYOUT);
        let skin = mad_skin::make(&screen.skin);
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
            KeyCode::Enter => {
                Some(StateTransition::PlayLevel(PlayLevelState {
                    comes_from_edit: false,
                    path: None,
                    level: Box::new(self.loaded_campaign.levels[self.selection].level.clone()),
                }))
            }
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




