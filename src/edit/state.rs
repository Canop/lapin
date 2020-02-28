
use {
    anyhow,
    crate::{
        level::Level,
        fromage::EditSubCommand,
        serde,
    },
    std::{
        boxed::Box,
        convert::{
            TryFrom,
        },
        path::PathBuf,
    },
};

pub struct EditLevelState {
    pub path: PathBuf,
    pub level: Box<Level>,
}

impl TryFrom<EditSubCommand> for EditLevelState {
    type Error = anyhow::Error;
    fn try_from(psc: EditSubCommand) -> Result<Self, Self::Error> {
        let path = psc.path;
        debug!("opening level editor on {:?}", &path);
        let level = if path.exists() {
            serde::read_file(&path)?
            // FIXME call level validity checks here
        } else {
            debug!("non existing file : starting with a clean board");
            Level::default()
        };
        Ok(EditLevelState {
            path,
            level: Box::new(level),
        })
    }
}

