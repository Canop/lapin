use {
    crate::{
        actor::*,
        consts::*,
        item::*,
    },
    std::fmt,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Ink {
    EraseTerrain,
    Terrain(Cell),
    EraseItem,
    Item(ItemKind),
    EraseActor,
    Actor(ActorKind),
}

pub static TERRAIN_INKS: &[Ink] = &[
    Ink::EraseTerrain,
    Ink::Terrain(FIELD),
    Ink::Terrain(SAND),
    Ink::Terrain(WALL),
    Ink::Terrain(GRASS),
    Ink::Terrain(WATER),
];
pub static ITEM_INKS: &[Ink] = &[
    Ink::EraseItem,
    Ink::Item(ItemKind::Carrot),
    Ink::Item(ItemKind::Wine),
];
pub static ACTOR_INKS: &[Ink] = &[
    Ink::EraseActor,
    Ink::Actor(ActorKind::Lapin),
    Ink::Actor(ActorKind::Knight),
    Ink::Actor(ActorKind::Wolf),
    Ink::Actor(ActorKind::Fox),
    Ink::Actor(ActorKind::Hunter),
    Ink::Actor(ActorKind::Sheep),
];

impl fmt::Display for Ink {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Ink::EraseTerrain => write!(f, "remove terrain"),
            Ink::Terrain(c) => write!(f, "{}", TERRAIN_NAMES[*c as usize]),
            Ink::EraseItem => write!(f, "remove item"),
            Ink::Item(ik) => ik.fmt(f),
            Ink::EraseActor => write!(f, "remove actor"),
            Ink::Actor(ak) => ak.fmt(f),
        }
    }
}
