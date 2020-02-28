
use {
    anyhow::Result,
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
    // TODO remove this ? It's not yet used
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
    ) -> Result<()>{
        use StateTransition::*;
        let mut current_transition = fromage.clone().try_into()?;
        loop {
            let next_transition =  match &current_transition {
                EditLevel (state) => {
                    edit::run(w, dam, state, &fromage)?
                }
                PlayLevel (state) => {
                    play::run(w, dam, state)?
                }
                Quit => {
                    return Ok(());
                }
            };
            self.previous_transitions.push(current_transition);
            current_transition = next_transition;
        }
    }
}

