
use {
    crossterm::{
    },
    crate::{
        game_runner,
        message_runner,
        io::W,
    },
};

pub enum AppState {
    Level,  // there might be a level id or something later
    Message(String),
    Quit,
}

pub fn run(w: &mut W) {
    let mut state = Ok(AppState::Level);
    loop {
        state = match state {
            Ok(AppState::Level) => game_runner::run(w),
            Ok(AppState::Message(s)) => message_runner::run(w, s),
            Ok(AppState::Quit) => { return; }
            Err(e) => {
                println!("damn: {:?}", e);
                return; // we just quit
            }
        }
    }
}

