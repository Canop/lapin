
use {
    anyhow::Result,
    crate::{
        app_state::StateTransition,
        fromage::*,
        level::Level,
        persist::{
            self,
            Bag,
        },
        test_level,
    },
    std::{
        boxed::Box,
        convert::TryFrom,
        path::PathBuf,
    },
};


pub struct PlayLevelState {
    pub comes_from_edit: bool,
    pub path: Option<PathBuf>,
    pub level: Box<Level>,
}

pub struct PlayCampaignState { // rename as ChooseLevelState ?
    pub path: PathBuf,
    pub bag: Box<Bag>, // a bag assumed to contain a campaign
}

impl Default for PlayLevelState {
    fn default() -> Self {
        Self {
            comes_from_edit: false,
            path: None,
            level: Box::new(test_level::build()),
        }
    }
}

impl TryFrom<PlayCommand> for PlayLevelState {
    type Error = anyhow::Error;
    fn try_from(psc: PlayCommand) -> Result<Self, Self::Error> {
        let level = if let Some(path) = &psc.path {
            persist::read_file(path)?
            // FIXME call level validity checks here
        } else {
            test_level::build()
        };
        Ok(PlayLevelState {
            comes_from_edit: false,
            path: psc.path,
            level: Box::new(level),
        })
    }
}

pub fn play_state_transition(psc: PlayCommand) -> Result<StateTransition> {
    if let Some(path) = psc.path {
        let mut bag: Bag = persist::read_file(&path)?;
        if let Some(level) = bag.as_sole_level() {
            Ok(StateTransition::PlayLevel(PlayLevelState {
                comes_from_edit: false,
                path: Some(path),
                level: Box::new(level),
            }))
        } else if bag.is_campaign() {
            Ok(StateTransition::PlayCampaign(PlayCampaignState {
                path,
                bag: Box::new(bag),
            }))
        } else {
            Err(anyhow!("nothing found in bag"))
        }
    } else {
        Ok(StateTransition::PlayLevel(PlayLevelState {
            comes_from_edit: false,
            path: None,
            level: Box::new(test_level::build()),
        }))
    }
}
