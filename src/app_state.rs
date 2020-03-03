
use {
    anyhow,
    crate::{
        fromage::*,
        play::{
            self,
            PlayCampaignState,
            PlayLevelState,
        },
        edit::EditLevelState,
    },
    std::{
        convert::{
            TryFrom,
            TryInto,
        },
    },
};

pub enum StateTransition {
    EditLevel(EditLevelState),
    PlayCampaign(PlayCampaignState),
    PlayLevel(PlayLevelState),
    Quit,
}

impl Default for StateTransition {
    fn default() -> Self {
        StateTransition::PlayLevel(PlayLevelState::default())
    }
}

impl TryFrom<Fromage> for StateTransition {
    type Error = anyhow::Error;
    fn try_from(fromage: Fromage) -> Result<Self, Self::Error> {
        Ok(match fromage.command {
            Some(Command::Edit ( esc )) => StateTransition::EditLevel(esc.try_into()?),
            Some(Command::Play ( psc )) => play::play_state_transition(psc)?,
            _ => Self::default(),
        })
    }
}
