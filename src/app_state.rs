
use {
    anyhow,
    crate::{
        fromage::*,
        play::PlayLevelState,
        edit::EditLevelState,
    },
    std::{
        convert::{
            TryFrom,
            TryInto,
        },
    },
};

/// the immutable context, read once from launch
/// arguments
//pub struct Context {
//    pub path: Option<PathBuf>,
//    pub output_format: Option<SerdeFormat>,
//}
//impl From<&Fromage> for Context {
//
//}

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
