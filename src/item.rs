
use {
    crate::{
        skin::*,
    },
};

#[derive(Debug, Clone, Copy)]
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

#[derive(Debug, Clone, Copy)]
pub struct Item {
    pub kind: ItemKind,
}
