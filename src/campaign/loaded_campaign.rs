
use {
    anyhow::Result,
    crate::{
        persist::{
            Level,
            Bag,
        },
        win_db,
    },
    std::{
        path::Path,
    },
    super::*,
};

/// a ready to play or display campaign, with resources loaded.
///
/// A loaded campaign packs
/// - the level found in bag or on disk
/// - win info related to the user
pub struct LoadedCampaign {
    pub campaign: Campaign,
    pub levels: Vec<LoadedLevel>,
}

#[derive(Debug, Clone, Copy)]
pub enum LoadOrigin {

    /// the level comes from the same file than the campaign
    Bag,

    /// the level comes from an other file, found by key
    External,

}

pub struct LoadedLevel {
    pub level: Level,
    pub won: bool,
}

fn load_external_level(
    campaign_path: &Path,
    level_key: &str,
) -> Result<Option<Level>> {
    debug!("looking for level {:?} in {:?}", level_key, campaign_path);
    for sf in persist::FORMATS {
        let path = campaign_path.with_file_name(level_key).with_extension(sf.key());
        debug!("trying path {:?}", &path);
        if path.exists() {
            let mut bag: Bag = persist::read_file(&path)?;
            if let Some(level) = bag.as_sole_level() {
                return Ok(Some(level));
            }
        }
    }
    Ok(None)
}

impl LoadedCampaign {

    /// load the levels
    ///
    /// Don't check if some levels have been won (call check_wins for that).
    /// This should only be called after you've checked the bag does contain
    /// a campaign
    pub fn load(
        path: &Path,
        mut bag: Bag,
        preferred_origin: LoadOrigin,
    ) -> Result<Self> {
        let campaign = bag.campaigns.pop().expect("tried to load a bag without campaign");
        let mut levels = Vec::new();
        for key in &campaign.levels {
            let level = match preferred_origin {
                LoadOrigin::Bag => match bag.levels.remove(key) {
                    Some(level) => level,
                    None => load_external_level(path, key)?
                        .ok_or(anyhow!("Level {:?} not found", key))?,
                }
                LoadOrigin::External => match load_external_level(path, key)? {
                    Some(level) => level,
                    None => bag.levels.remove(key)
                        .ok_or(anyhow!("Level {:?} not found", key))?,
                }
            };
            levels.push(LoadedLevel {
                won: false,
                level,
            });
        }
        if levels.is_empty() {
            Err(anyhow!("Empty campaign"))
        } else {
            Ok(Self {
                campaign,
                levels,
            })
        }
    }

    /// build a loaded campaign assuming the bag already
    /// contains all the levels.
    pub fn from_packed_bag(mut bag: Bag) -> Result<Self> {
        let campaign = bag.campaigns.pop().expect("bag without campaign");
        let mut levels = Vec::new();
        for key in &campaign.levels {
            let level = bag.levels.remove(key)
                .ok_or(anyhow!("Level {:?} not in bag", key))?;
            levels.push(LoadedLevel {
                won: false,
                level,
            });
        }
        if levels.is_empty() {
            Err(anyhow!("Empty campaign"))
        } else {
            Ok(Self {
                campaign,
                levels,
            })
        }
    }

    /// check in the win database to determine what levels have been won.
    ///
    /// This can (and should) be called several times, when it may have
    /// changed.
    pub fn check_wins(&mut self) -> Result<()> {
        if let Ok(win_file) = win_db::WinFile::load() {
            for level in self.levels.iter_mut() {
                let signature = win_db::Signature::new(&level.level)?;
                level.won = win_file.has_win(&signature);
            }
        }
        Ok(())
    }

    pub fn name(&self) -> &str {
        &self.campaign.name
    }
}
