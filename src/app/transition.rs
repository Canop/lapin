
/// a transition from one screen to another one.
///
/// The transition only contains the information needed
/// to go from a known state to the next one
#[derive(Debug, Clone, Copy)]
pub enum StateTransition {
    PlayLevel {
        level_idx: usize,
    },
    Help,
    Back,
    Quit,
}

