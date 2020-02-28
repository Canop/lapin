use {
    anyhow::Result,
    crate::{
        app_state::StateTransition,
        fromage::*,
        io::W,
        layout::Layout,
        task_sync::*,
    },
};

mod drawing_action;
mod drawing_history;
mod head_panel;
mod ink;
mod inkwell;
mod level_editor;
mod pen;
mod pen_panel;
mod state;

pub use state::EditLevelState;

pub const LAYOUT: Layout = Layout {
    header_height: 3,
    pen_panel_height: 3,
    status_height: 1,
};

pub fn run(
    w: &mut W,
    dam: &mut Dam,
    state: &EditLevelState,
    fromage: &Fromage,
) -> Result<StateTransition> {
    let mut level_editor = level_editor::LevelEditor::new(state, fromage);
    //let output_format = if let Some(
    level_editor.run(w, dam)
}

