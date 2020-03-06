/// manage the state+transition based TUI of Lapin
///

mod app;
mod fromage;
mod initial_state;
mod state;
mod task_sync;
mod transition;

pub use {
    app::App,
    fromage::*,
    state::State,
    task_sync::Dam,
    transition::StateTransition,
};
