
use {
    std::io::{
        BufWriter,
        Stderr,
    },
};

mod animate;
mod board_drawer;
mod layout;
pub mod mad_skin;
mod screen;
mod skin;
mod status;

pub type W = BufWriter<Stderr>;

pub use {
    board_drawer::BoardDrawer,
    layout::{
        Areas,
        Layout,
    },
    screen::Screen,
    skin::Skin,
    status::Status,
};

