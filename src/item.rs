
use {
    crate::{
        skin::*,
    },
};

#[derive(Debug, Clone, Copy)]
pub enum ItemKind {
    Carrot,
}
impl ItemKind {
    pub fn skin(self, skin: &Skin) -> FgSkin {
        match self {
            ItemKind::Carrot => skin.carrot,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Item {
    pub kind: ItemKind,
}
