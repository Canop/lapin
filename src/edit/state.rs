
use {
    anyhow,
    crate::{
        level::Level,
        fromage::EditSubCommand,
    },
    std::{
        boxed::Box,
        convert::{
            TryFrom,
        },
        fs::{
            File,
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
            debug!("trying to deserialize the file");
            let file = File::open(&path)?;
            deser = serde_json::from_reader(file)
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

