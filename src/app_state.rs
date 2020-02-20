
use {
    anyhow,
    crate::{
        fromage::*,
        level::Level,
        test_level,
        play::PlayLevelState,
        edit::EditLevelState,
    },
    std::{
        boxed::Box,
        convert::{
            TryFrom,
            TryInto,
        },
        fs::{
            File,
        },
        path::PathBuf,
    },
};

pub enum StateTransition {
    EditLevel(EditLevelState),
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
        Ok(match fromage.sub {
            Some(SubCommand::Edit ( esc )) => StateTransition::EditLevel(esc.try_into()?),
            Some(SubCommand::Play ( psc )) => StateTransition::PlayLevel(psc.try_into()?),
            _ => Self::default(),
        })
    }
}
