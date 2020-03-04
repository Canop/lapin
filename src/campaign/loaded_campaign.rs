
use {
    anyhow::Result,
    crate::{
        level::Level,
        persist::{
            Bag,
        },
        win_db,
    },
    std::{
        path::Path,
    },
    super::*,
};

/// A loaded campaign packs
/// - the level found in bag or on disk
/// - win info related to the user
pub struct LoadedCampaign {
    campaign: Campaign,
    pub levels: Vec<LoadedLevel>,
}

pub struct LoadedLevel {
    pub level: Level,
    pub won: bool,
}

fn load_external_level(
    campaign_path: &Path,
    level_key: &str,
) -> Result<Level> {
    debug!("looking for level {:?} in {:?}", level_key, campaign_path);
    for sf in persist::FORMATS {
        let path = campaign_path.with_file_name(level_key).with_extension(sf.key());
        debug!("trying path {:?}", &path);
        if path.exists() {
            let mut bag: Bag = persist::read_file(&path)?;
            if let Some(level) = bag.as_sole_level() {
                return Ok(level);
            }
        }
    }
    Err(anyhow!("Level {:?} not found", level_key))
}

impl LoadedCampaign {
    pub fn load(
        path: &Path,
        mut bag: Bag,
    ) -> Result<Self> {
        // this should only be called after you've checked the bag does contain
        // a campaign
        let campaign = bag.campaigns.pop().expect("tried to load a bag without campaign");
        let mut levels = Vec::new();
        let win_file = win_db::WinFile::load().ok();
        for key in &campaign.levels {
            let level = match bag.levels.remove(key) {
                Some(level) => level,
                None => load_external_level(path, key)?,
            };
            let signature = win_db::Signature::new(&level)?;
            levels.push(LoadedLevel {
                won: win_file.as_ref().map_or(false, |wf| wf.has_win(&signature)),
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
    pub fn name(&self) -> &str {
        &self.campaign.name
    }
}
