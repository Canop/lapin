use {
    anyhow::Result,
    crate::{
        app::AppState,
        fromage::EditSubCommand,
        io::W,
        layout::Layout,
        task_sync::*,
    },
};

mod level_editor;
mod pen;
mod selector;

pub const LAYOUT: Layout = Layout {
    selector_height: 3,
    status_height: 1,
};

pub fn run(
    w: &mut W,
    dam: &mut Dam,
    esc: EditSubCommand,
) -> Result<AppState> {
    let mut level_editor = level_editor::LevelEditor::new(esc)?;
    level_editor.run(w, dam)
}

