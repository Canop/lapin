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
mod game_runner;
mod help_text;
mod level_chooser;
mod move_result;
mod state;
mod world;

pub use move_result::*;
pub use state::{
    play_state_transition,
    PlayCampaignState,
    PlayLevelState,
};
pub use world::*;


pub const LAYOUT: Layout = Layout {
    header_height: 0,
    pen_panel_height: 0,
    status_height: 1,
};

pub fn play_level(
    w: &mut W,
    dam: &mut Dam,
    state: &PlayLevelState,
) -> Result<StateTransition> {
    let mut game_runner = game_runner::GameRunner::new(state)?;
    game_runner.run(w, dam)
}

pub fn play_campaign(
    w: &mut W,
    dam: &mut Dam,
    state: PlayCampaignState,
) -> Result<StateTransition> {
    let mut chooser = level_chooser::LevelChooser::new(state)?;
    chooser.run(w, dam)
}
