
use {
    crate::{
        skin::*,
    },
    serde::{Serialize, Deserialize},
};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ItemKind {
    Carrot,
    Wine,
}
impl ItemKind {
    pub fn skin(self, skin: &Skin) -> FgSkin {
        match self {
            ItemKind::Carrot => skin.carrot,
            ItemKind::Wine => skin.wine,
        }
    }
}

pub static ITEMS: &[ItemKind] = &[ItemKind::Carrot, ItemKind::Wine];

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Item {
    pub kind: ItemKind,
}
