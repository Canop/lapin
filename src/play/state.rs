
use {
    anyhow,
    crate::{
        fromage::*,
        level::Level,
        serde,
        test_level,
    },
    std::{
        boxed::Box,
        convert::{
            TryFrom,
        },
        path::PathBuf,
    },
};


pub struct PlayLevelState {
    pub comes_from_edit: bool,
    pub path: Option<PathBuf>, // TODO remove (we can just keep the path)
    pub level: Box<Level>,
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

impl TryFrom<PlaySubCommand> for PlayLevelState {
    type Error = anyhow::Error;
    fn try_from(psc: PlaySubCommand) -> Result<Self, Self::Error> {
        let level = if let Some(path) = &psc.path {
            serde::read_file(path)?
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
