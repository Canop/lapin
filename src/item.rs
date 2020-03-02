
use {
    crate::{
        skin::*,
    },
    serde::{Serialize, Deserialize},
    std::{
        fmt,
        hash::Hash,
    },
    termimad::{
        StyledChar,
    },
};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ItemKind {
    Carrot,
    Wine,
}
impl ItemKind {
    pub fn skin(self, skin: &Skin) -> &StyledChar {
        match self {
            ItemKind::Carrot => &skin.carrot,
            ItemKind::Wine => &skin.wine,
        }
    }
}

impl fmt::Display for ItemKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ItemKind::Carrot => write!(f, "carrot"),
            ItemKind::Wine => write!(f, "wine bottle"),
        }
    }
}

pub static ITEMS: &[ItemKind] = &[ItemKind::Carrot, ItemKind::Wine];

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Hash)]
pub struct Item {
    pub kind: ItemKind,
}
