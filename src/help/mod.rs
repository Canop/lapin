
use crate::display::Layout;

mod help_text;
mod state;

pub use state::HelpState;

pub const LAYOUT: Layout = Layout {
    header_height: 0,
    pen_panel_height: 0,
    status_height: 1,
};

/// return a help screen with the standard text and layout
pub fn default_view() -> HelpState {
    HelpState::new(help_text::MARKDOWN, LAYOUT)
}
