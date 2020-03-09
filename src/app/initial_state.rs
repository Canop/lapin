use {
    anyhow::Result,
    crate::{
        campaign::{
            LoadedCampaign,
            LoadOrigin,
        },
        choose,
        edit,
        included,
        play,
        persist::{
            self,
            Bag,
        },
    },
    std::{
        convert::TryFrom,
    },
    super::*,
};


/// Determine the first state
pub fn make(fromage: &Fromage) -> Result<Box<dyn State>> {
    match &fromage.command {
        Some(Command::Edit ( ec )) => Ok(Box::new(
            edit::LevelEditor::try_from(ec)?
        )),
        Some(Command::Play ( pc )) => play_state(pc),
        _ => default_state(),
    }
}

/// A default state for when there's nothing in the fromage
fn default_state() -> Result<Box<dyn State>> {
    Ok(Box::new(choose::ChooseLevelState::new(
        included::loaded_campaign()?
    )?))
}

/// compute the relevant initial state for a `lapin play` command
/// (may be choosing a level or directly playing it)
fn play_state(pc: &PlayCommand) -> Result<Box<dyn State>> {
    if let Some(path) = &pc.path {
        let mut bag: Bag = persist::read_file(&path)?;
        if let Some(level) = bag.as_sole_level() {
            Ok(Box::new(play::PlayLevelState::new(&level, None)?))
        } else if bag.is_campaign() {
            let loaded_campaign = LoadedCampaign::load(&path, bag, LoadOrigin::Bag)?;
            Ok(Box::new(choose::ChooseLevelState::new(loaded_campaign)?))
        } else {
            Err(anyhow!("nothing found in bag"))
        }
    } else {
        default_state()
    }
}
