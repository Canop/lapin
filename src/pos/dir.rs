
use {
    serde::{Serialize, Deserialize},
};

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
