use {
    crate::{
        display::Layout,
    },
};

mod drawing_action;
mod drawing_history;
mod head_panel;
mod ink;
mod inkwell;
mod pen;
mod pen_panel;
mod state;

pub use state::LevelEditor;

pub static LABEL: &str = "editor";

pub const LAYOUT: Layout = Layout {
    header_height: 3,
    pen_panel_height: 3,
    status_height: 1,
};


