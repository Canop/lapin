
use crate::display::Layout;

mod help_text;
mod help_view;

pub use help_view::View;

pub const LAYOUT: Layout = Layout {
    header_height: 0,
    pen_panel_height: 0,
    status_height: 1,
};

/// return a help screen with the standard text and layout
pub fn default_view() -> View {
    View::new(help_text::MARKDOWN, LAYOUT)
}
