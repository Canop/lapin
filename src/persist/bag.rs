
use {
    crate::{
        campaign::Campaign,
    },
    fnv::FnvHashMap,
    serde::{Serialize, Deserialize},
    super::Level,
};


/// a bag of data which may be written to a file
/// and read back. It may be a level or a campaign
/// (with or without the levels it needs).
///
/// The format supports a bag containing several
/// campaigns so that files don't become invalid
/// in the future if we support this but right now
/// the code will only use the first campaign
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Bag {
    pub campaigns: Vec<Campaign>,
    pub levels: FnvHashMap<String, Level>,
}

impl Bag {
    pub fn is_empty(&self) -> bool {
        self.campaigns.is_empty() && self.levels.is_empty()
    }
    pub fn is_campaign(&self) -> bool {
        self.campaigns.len() > 0
    }
    pub fn as_sole_campaign(&mut self) -> Option<Campaign> {
        self.campaigns.pop()
    }
    /// In the precise case the bag only contains one level,
    /// return this level
    pub fn as_sole_level(&mut self) -> Option<Level> {
        if self.levels.len() == 1 {
            self.levels.drain().next().map(|(_,l)| l)
        } else {
            None
        }
    }
}

impl From<Level> for Bag {
    fn from(level: Level) -> Self {
        let mut bag = Bag::default();
        bag.levels.insert(String::new(), level);
        bag
    }
}

impl From<Campaign> for Bag {
    fn from(campaign: Campaign) -> Self {
        let mut bag = Bag::default();
        bag.campaigns.push(campaign);
        bag
    }
}
