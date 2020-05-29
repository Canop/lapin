
use {
    serde::{Serialize, Deserialize},
};

/// All Compass directions.
/// The first four ones are called "quadrant" dir in the rest of the code.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Dir {
    Up,
    Right,
    Down,
    Left,
    UpRight,
    RightDown,
    DownLeft,
    LeftUp
}

pub static ALL_DIRS: &[Dir] = &[
    Dir::Up,
    Dir::Right,
    Dir::Down,
    Dir::Left,
    Dir::UpRight,
    Dir::RightDown,
    Dir::DownLeft,
    Dir::LeftUp
];

impl Dir {
    pub fn is_vertical(self) -> bool {
        match self {
            Self::Up | Self::Down => true,
            _ => false,
        }
    }
}
