use {
    anyhow::Result,
    crate::{
        campaign::{
            LoadedCampaign,
        },
        choose,
        edit,
        play,
        persist::{
            self,
            Bag,
        },
        test_level,
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
    let level = test_level::build();
    Ok(Box::new(play::PlayLevelState::new(&level, None)?))
}

fn play_state(pc: &PlayCommand) -> Result<Box<dyn State>> {
    if let Some(path) = &pc.path {
        let mut bag: Bag = persist::read_file(&path)?;
        if let Some(level) = bag.as_sole_level() {
            Ok(Box::new(play::PlayLevelState::new(&level, None)?))
        } else if bag.is_campaign() {
            let loaded_campaign = LoadedCampaign::load(&path, bag)?;
            Ok(Box::new(choose::ChooseLevelState::new(loaded_campaign)?))
        } else {
            Err(anyhow!("nothing found in bag"))
        }
    } else {
        default_state()
    }
}
