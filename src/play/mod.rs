use {
    anyhow::Result,
    crate::{
        app::AppState,
        fromage::PlaySubCommand,
        io::W,
        layout::Layout,
        task_sync::*,
    },
};

mod animate;
mod command;
mod game_runner;
mod move_result;
mod world;

pub use move_result::*;
pub use command::*;
pub use world::*;

pub const LAYOUT: Layout = Layout {
    selector_height: 0,
    status_height: 1,
};

pub fn run(
    w: &mut W,
    dam: &mut Dam,
    psc: PlaySubCommand,
) -> Result<AppState> {
    let mut game_runner = game_runner::GameRunner::new(psc)?;
    game_runner.run(w, dam)
}
