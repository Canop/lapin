
use {
    crate::{
        editor::LevelEditor,
        game_runner::GameRunner,
        io::W,
        fromage::*,
        message_runner,
        task_sync::*,
    },
};

#[derive(Debug)]
pub enum AppState {
    PlayLevel,  // there might be a level id or something later
    EditLevel,  // there might be a level id or something later
    Message(String, bool),
    Quit,
}

pub fn run(
    w: &mut W,
    dam: &mut Dam,
    fromage: Fromage,
) {
    use AppState::*;
    let mut state = Ok(match fromage.sub {
        Some(SubCommand::Edit { .. }) => EditLevel,
        _ => PlayLevel,
    });
    debug!("initial state: {:?}", &state);
    loop {
        state = match state {
            Ok(EditLevel) => {
                let mut level_editor = LevelEditor::new();
                level_editor.run(w, dam)
            }
            Ok(PlayLevel) => {
                let mut game_runner = GameRunner::new();
                game_runner.run(w, dam)
            }
            Ok(Message(s, good)) => {
                message_runner::run(w, s, good, dam)
            }
            Ok(Quit) => { return; }
            Err(e) => {
                println!("damn: {:?}", e);
                return; // we just quit
            }
        }
    }
}

