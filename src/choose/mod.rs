
use {
    crate::{
        display::Layout,
    },
};

mod state;

pub use state::ChooseLevelState;

pub const LAYOUT: Layout = Layout {
    header_height: 0,
    pen_panel_height: 0,
    status_height: 1,
};
