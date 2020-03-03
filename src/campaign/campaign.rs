
use {
    serde::{Serialize, Deserialize},
};


/// a campaign doesn't actually contains the levels,
/// only their reference (name)
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Campaign {

    pub name: String,

    pub description: String,

    /// levels in the order they should be done
    pub levels: Vec<String>,

}
