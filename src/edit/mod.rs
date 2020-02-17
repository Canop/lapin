use {
    anyhow::Result,
    crate::{
        app::AppState,
        fromage::EditSubCommand,
        io::W,
        task_sync::*,
    },
};

mod level_editor;
mod pen;

pub fn run(
    w: &mut W,
    dam: &mut Dam,
    esc: EditSubCommand,
) -> Result<AppState> {
    let mut level_editor = level_editor::LevelEditor::new(esc)?;
    level_editor.run(w, dam)
}

