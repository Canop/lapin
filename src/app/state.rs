
use {
    anyhow::Result,
    crate::{
        display::W,
        level::Level,
    },
    super::{
        Dam,
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
        w: &mut W,
        dam: &mut Dam,
    ) -> Result<StateTransition>;

    fn get_level(
        &self,
        level_idx: usize,
    ) -> Option<Level>;
}

