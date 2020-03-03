/// Execute all the functions which need the terminal UI
use {
    anyhow::Result,
    crate::{
        app_state::StateTransition,
        edit,
        fromage::*,
        io::W,
        play,
        task_sync::*,
    },
    std::convert::TryInto,
};

pub struct App {
}

impl App {
    pub fn new() -> Self {
        Self {
        }
    }
    pub fn run(
        &mut self,
        w: &mut W,
        dam: &mut Dam,
        fromage: Fromage,
    ) -> Result<()> {
        use StateTransition::*;
        debug!("fromage: {:?}", &fromage);
        let mut current_transition = fromage.clone().try_into()?;
        loop {
            let next_transition =  match current_transition {
                EditLevel (state) => {
                    edit::run(w, dam, &state, &fromage)?
                }
                PlayLevel (state) => {
                    play::play_level(w, dam, &state)?
                }
                PlayCampaign(state) => {
                    play::play_campaign(w, dam, state)?
                }
                Quit => {
                    return Ok(());
                }
            };
            current_transition = next_transition;
        }
    }
}

