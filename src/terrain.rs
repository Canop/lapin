
use {
    crate::{
        skin::*,
    },
    crossterm::{
        style::{
            Color,
            SetBackgroundColor,
        },
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

// TODO define all those with a macro

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Terrain {
    Mud,
    Stone,
    Grass,
    Water,
    Sand,
}

pub static TERRAINS: &[Terrain] = &[
    Terrain::Mud,
    Terrain::Stone,
    Terrain::Grass,
    Terrain::Water,
    Terrain::Sand,
];

impl fmt::Display for Terrain {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Terrain::Mud => write!(f, "ground"),
            Terrain::Stone => write!(f, "stone"),
            Terrain::Grass => write!(f, "grass"),
            Terrain::Water => write!(f, "water"),
            Terrain::Sand => write!(f, "sand"),
        }
    }
}
impl Terrain {
    pub fn bg(self, skin: &Skin) -> Color {
        match self {
            Terrain::Mud => skin.mud,
            Terrain::Stone => skin.stone,
            Terrain::Grass => skin.grass,
            Terrain::Water => skin.water,
            Terrain::Sand => skin.sand,
        }
    }
    pub fn bg_as_styled_char(&self, skin: &Skin) -> StyledChar {
        StyledChar::from_fg_char(self.bg(skin), '█')
    }
    pub fn bg_command(&self, skin: &Skin) -> SetBackgroundColor {
        SetBackgroundColor(self.bg(skin))
    }
}
