use {
    crate::{
        app::AppState,
        board::*,
        command::Command,
        draw_board::BoardDrawer,
        io::W,
        pos::*,
        screen::Screen,
        status::Status,
        task_sync::*,
        test_level,
        world::*,
    },
    anyhow::Result,
    crossterm::{
        event::{
            KeyCode,
            KeyEvent,
        },
    },
    termimad::{
        Event,
    },
};

pub struct GameRunner {
    pub board: Board,
}

impl GameRunner {
    pub fn new() -> Self {
        let board = test_level::build();
        Self {
            board,
        }
    }

    fn handle_key_event(
        &mut self,
        code: KeyCode,
    ) -> Option<AppState> {
        match Command::from(code) {
            None => None,
            Some(Command::Quit) => {
                Some(AppState::Quit)
            }
            Some(cmd) => {
                let move_result = self.board.apply_player_move(cmd);
                next_state(move_result)
            }
        }
    }

    fn write_status(
        &mut self,
        w: &mut W,
        screen: &Screen,
    ) -> Result<()> {
        Status::from_message(mad_inline!(
            "Hit arrows to move, *q* to quit"
        ))
        .display(w, screen)
    }

    /// return the next state
    pub fn run(
        &mut self,
        w: &mut W,
        dam: &mut Dam,
    ) -> Result<AppState> {
        let mut screen = Screen::new()?;
        self.write_status(w, &screen)?;
        let mut seed = 0;
        loop {
            let mut bd = BoardDrawer::new(&self.board, w, &screen);
            bd.draw()?;
            let next_state = match self.board.current_player {
                Player::Lapin => {
                    let event = dam.next_event().unwrap();
                    dam.unblock();
                    match event {
                        Event::Key(KeyEvent { code, .. }) => {
                            self.handle_key_event(code)
                        }
                        Event::Resize(width, height) => {
                            screen.set_terminal_size(width, height);
                            self.write_status(w, &screen)?;
                            None
                        }
                        Event::Click(x, y) => {
                            let sp = ScreenPos{ x, y };
                            debug!("click in {:?}", bd.pos_converter.to_real(sp));
                            None
                        }
                        _ => {
                            debug!("ignored event: {:?}", event);
                            None
                        }
                    }
                }
                Player::World => {
                    let world_player = WorldPlayer::new(&self.board, seed);
                    seed += 1;
                    let world_move = time!(Info, "world play", world_player.play());
                    bd.animate(&world_move, dam)?;
                    let move_result = self.board.apply_world_move(world_move);
                    next_state(move_result)
                }
            };
            if let Some(next_state) = next_state {
                let mut bd = BoardDrawer::new(&self.board, w, &screen);
                bd.draw()?;
                return Ok(next_state);
            }
        }
    }
}

fn next_state(move_result: MoveResult) -> Option<AppState> {
    match move_result {
        MoveResult::PlayerWin => {
            Some(AppState::Message("You **WIN!**".to_string(), true))
        }
        MoveResult::PlayerLose => {
            Some(AppState::Message("You **LOSE!**".to_string(), false))
        }
        _ => None,
    }
}


