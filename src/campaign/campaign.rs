
use {
    serde::{Serialize, Deserialize},
};

/// a campaign doesn't actually contains the levels,
/// only their reference (name)
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
