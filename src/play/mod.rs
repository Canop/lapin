use {
    anyhow::Result,
    crate::{
        app_state::StateTransition,
        fromage::PlaySubCommand,
        io::W,
        layout::Layout,
        level::Level,
        task_sync::*,
    },
};

mod animate;
mod command;
mod game_runner;
mod move_result;
mod state;
mod world;

pub use move_result::*;
pub use command::*;
pub use state::PlayLevelState;
pub use world::*;


pub const LAYOUT: Layout = Layout {
    selector_height: 0,
    status_height: 1,
};

pub fn run(
    w: &mut W,
    dam: &mut Dam,
    state: &PlayLevelState,
) -> Result<StateTransition> {
    let mut game_runner = game_runner::GameRunner::new(state)?;
    game_runner.run(w, dam)
}
