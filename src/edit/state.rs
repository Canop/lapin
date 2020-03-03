
use {
    anyhow,
    crate::{
        level::Level,
        fromage::EditCommand,
        persist::{
            self,
            Bag,
        },
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

impl TryFrom<EditCommand> for EditLevelState {
    type Error = anyhow::Error;
    fn try_from(psc: EditCommand) -> Result<Self, Self::Error> {
        let path = psc.path;
        debug!("opening level editor on {:?}", &path);
        let level = if path.exists() {
            let mut bag: Bag = persist::read_file(&path)?;
            if let Some(level) = bag.as_sole_level() {
                level
            } else {
                return Err(anyhow!("Only single level files can be edited with this version of Lapin"));
            }
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

