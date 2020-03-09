
use {
    serde::{Serialize, Deserialize},
};


/// a campaign, that is mostly a list of reference
/// to levels.
///
/// A campaign doesn't actually contains the levels,
/// only their reference (name) which can be the stem
/// of a file or the key in the same Bag
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Campaign {

    pub name: String,

    /// a description, not yet used, but which could be in the future
    /// (reserved for easier compatibility)
    pub description: String,

    /// not yet used but should be in the future
    pub allow_all_levels: bool,

    /// levels in the order they should be done
    pub levels: Vec<String>,

}
