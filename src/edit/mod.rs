use {
    anyhow::Result,
    crate::{
        app_state::StateTransition,
        io::W,
        level::Level,
        layout::Layout,
        task_sync::*,
    },
    std::{
        path::Path,
    },
};

mod level_editor;
mod pen;
mod selector;
mod state;

pub use state::EditLevelState;

pub const LAYOUT: Layout = Layout {
    selector_height: 3,
    status_height: 1,
};

pub fn run(
    w: &mut W,
    dam: &mut Dam,
    state: &EditLevelState,
) -> Result<StateTransition> {
    let mut level_editor = level_editor::LevelEditor::new(state);
    level_editor.run(w, dam)
}

