use {
    anyhow::Result,
    crate::{
        display::W,
        help,
        play,
    },
    super::*,
};

/// Execute all the functions which need the terminal UI
pub struct App {
    /// a stack of states. The current one is the last in vec.
    states: Vec<Box<dyn State>>,
}

impl App {

    pub fn new() -> Self {
        Self {
            states: Vec::new(),
        }
    }

    fn current_state(&mut self) -> &mut dyn State {
        self.states
            .last_mut()
            .expect("No path has been pushed")
            .as_mut()
    }

    pub fn run(
        &mut self,
        w: &mut W,
        dam: &mut Dam,
        fromage: Fromage,
    ) -> Result<()> {
        debug!("fromage: {:?}", &fromage);
        use StateTransition::*;
        self.states.push(initial_state::make(&fromage)?);
        loop {
            let label = self.current_state().label();
            info!("opening state {:?}", label);
            match self.current_state().run(w, dam)? {
                PlayLevel{level_idx} => {
                    if let Some(level) = self.current_state().get_level(level_idx) {
                        self.states.push(Box::new(
                            play::PlayLevelState::new(
                                &level,
                                Some(label),
                            )?
                        ));
                    }
                }
                Help => {
                    self.states.push(Box::new(
                        help::default_view()
                    ));
                }
                Back => {
                    self.states.pop();
                }
                Quit => {
                    break;
                }
            };
        }
        Ok(())
    }
}

