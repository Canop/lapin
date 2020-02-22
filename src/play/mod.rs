use {
    anyhow::Result,
    crate::{
        app_state::StateTransition,
        io::W,
        layout::Layout,
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
    header_height: 0,
    pen_panel_height: 0,
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
