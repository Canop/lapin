
use {
    anyhow::Result,
    crate::{
        level::Level,
    },
    super::{
        Context,
        transition::StateTransition,
    },
};


/// One of the screens of Lapin.
pub trait State {

    /// a label used as hint for user of the state where
    /// they could get back
    fn label(&self) -> &'static str;

    /// display the state on screen and handled events until
    /// a transition must be done.
    ///
    /// This function can be called several times (if there's
    /// a back to a previous state, it's ran again).
    fn run(
        &mut self,
        con: &mut Context,
    ) -> Result<StateTransition>;

    /// provide a level (to the next state if it
    /// needs one which should come from the current one)
    fn get_level(
        &self,
        level_idx: usize,
    ) -> Option<Level>;
}

