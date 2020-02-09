use {
    crate::{
        app::AppState,
        board::*,
        command::Command,
        draw_board::BoardDrawer,
        io::W,
        screen::Screen,
        test_level,
        world,
    },
    anyhow::Result,
    crossterm::{
        event::{self, Event, KeyEvent},
    },
    std::io::Write,
};

pub struct GameRunner {
    pub board: Board,
    pub screen: Screen,
}

impl GameRunner {
    pub fn new() -> Result<Self> {
        let board = test_level::build();
        let screen = Screen::new()?;
        Ok(Self {
            board,
            screen,
        })
    }
    pub fn write(&self, w: &mut W) -> Result<()> {
        let mut bd = BoardDrawer::new(&self.board, w, &self.screen);
        bd.draw()?;
        Ok(())
    }
}

fn end_message(move_result: &MoveResult) -> Result<AppState> {
    match move_result {
        MoveResult::PlayerWin => {
            return Ok(AppState::Message("You WIN!   hit 'q' to quit".to_string()));
        }
        MoveResult::PlayerLose => {
            return Ok(AppState::Message("You LOSE!   hit 'q' to quit".to_string()));
        }
        _ => Err(anyhow!("Invalid Internal State"))

    }
}

/// return the next state
pub fn run(w: &mut W) -> Result<AppState> {
    let mut gr = GameRunner::new()?;
    loop {
        gr.write(w)?;
        w.flush()?;
        if let Ok(Event::Key(KeyEvent { code, .. })) = event::read() {
            match Command::from(code) {
                None => { }
                Some(Command::Quit) => break,
                Some(cmd) => {
                    let move_result = gr.board.apply_player_move(cmd);
                    let mut bd = BoardDrawer::new(&gr.board, w, &gr.screen);
                    bd.draw()?;
                    match &move_result {
                        MoveResult::Invalid => { continue; }
                        MoveResult::Ok => {
                            let world_move = world::play(&gr.board);
                            //debug!("world_move: {:?}", &world_move);
                            bd.animate(&world_move)?;
                            let move_result = gr.board.apply_world_move(world_move);
                            match move_result {
                                MoveResult::Ok => { continue; }
                                _ => { return end_message(&move_result); }
                            }
                        }
                        _ => { return end_message(&move_result); }
                    }
                }
            }
        }
    }
    Ok(AppState::Quit)
}
