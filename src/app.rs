
use {
    crate::{
        edit,
        fromage::*,
        io::W,
        play,
        task_sync::*,
    },
};

#[derive(Debug)]
pub enum AppState {
    PlayLevel(PlaySubCommand),
    EditLevel(EditSubCommand),
    Quit,
}

pub fn run(
    w: &mut W,
    dam: &mut Dam,
    fromage: Fromage,
) {
    use AppState::*;
    let mut state = Ok(match fromage.sub {
        Some(SubCommand::Edit ( esc )) => EditLevel(esc),
        Some(SubCommand::Play ( psc )) => PlayLevel(psc),
        _ => PlayLevel(PlaySubCommand::default()),
    });
    loop {
        debug!("app state: {:?}", &state);
        state = match state {
            Ok(EditLevel(esc)) => {
                edit::run(w, dam, esc)
            }
            Ok(PlayLevel(psc)) => {
                play::run(w, dam, psc)
            }
            Ok(Quit) => { return; }
            Err(e) => {
                println!("damn: {:?}", e);
                return; // we just quit
            }
        }
    }
}

