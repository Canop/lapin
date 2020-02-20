
use {
    crate::{
        app_state::StateTransition,
        edit,
        fromage::*,
        io::W,
        play,
        task_sync::*,
    },
    std::convert::TryInto,
};

pub struct App {
    // TODO remove this ?
    previous_transitions: Vec<StateTransition>,
}

impl App {
    pub fn new() -> Self {
        Self {
            previous_transitions: Vec::new(),
        }
    }
    pub fn run(
        &mut self,
        w: &mut W,
        dam: &mut Dam,
        fromage: Fromage,
    ) {
        use StateTransition::*;
        let mut current_transition = fromage.try_into();
        loop {
            match current_transition {
                Ok(transition) => {
                    let next_transition =  match &transition {
                        EditLevel (state) => {
                            edit::run(w, dam, state)
                        }
                        PlayLevel (state) => {
                            play::run(w, dam, state)
                        }
                        Quit => { return; }
                    };
                    self.previous_transitions.push(transition);
                    current_transition = next_transition;
                }
                Err(e) => {
                    println!("damn: {:?}", e);
                    return; // we just quit
                }
            }
        }
    }
}

