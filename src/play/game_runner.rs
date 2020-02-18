use {
    crate::{
        app::AppState,
        board::*,
        draw_board::BoardDrawer,
        fromage::PlaySubCommand,
        io::W,
        level::Level,
        pos::*,
        screen::Screen,
        status::Status,
        task_sync::*,
        test_level,
    },
    anyhow::Result,
    crossterm::{
        event::{
            KeyCode,
            KeyEvent,
        },
    },
    std::{
        fs::{
            File,
        },
        time::SystemTime,
    },
    super::*,
    termimad::{
        Event,
    },
};

pub struct GameRunner {
    board: Board,
    status: Status,
}

impl GameRunner {
    pub fn new(psc: PlaySubCommand) -> Result<Self> {
        let board = if let Some(path) = psc.path {
            let file = File::open(&path)?;
            let level: Level = serde_json::from_reader(file)?;
            // FIXME call validity checks here
            Board::from(&level)
        } else {
            Board::from(&test_level::build())
        };
        let status = Status::from_message(
            "Hit *arrows* to move, *q* to quit".to_string()
        );
        Ok(Self {
            board,
            status,
        })
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
                self.apply(move_result);
                None
            }
        }
    }

    fn apply(&mut self, move_result: MoveResult)  {
        match move_result {
            MoveResult::PlayerWin => {
                self.status = Status::from_message(
                    "You **WIN!**".to_string()
                );
            }
            MoveResult::PlayerLose => {
                self.status = Status::from_error(
                    "You **LOSE!**".to_string()
                );
            }
            _ => {}
        }
    }

    fn write_status(
        &mut self,
        w: &mut W,
        screen: &Screen,
    ) -> Result<()> {
        self.status.display(w, screen)
    }

    /// return the next state
    pub fn run(
        &mut self,
        w: &mut W,
        dam: &mut Dam,
    ) -> Result<AppState> {
        let mut screen = Screen::new(LAYOUT);
        let mut seed = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .map_or(0, |d| (d.as_secs()%7) as usize);
        loop {
            let mut bd = BoardDrawer::new(&self.board, w, &screen);
            bd.draw()?;
            self.write_status(w, &screen)?;
            if self.board.current_player == Player::World {
                let world_player = WorldPlayer::new(&self.board, seed);
                seed += 1;
                let world_move = time!(Info, "world play", world_player.play());
                let mut bd = BoardDrawer::new(&self.board, w, &screen);
                bd.animate(&world_move, dam)?;
                let move_result = self.board.apply_world_move(world_move);
                let mut bd = BoardDrawer::new(&self.board, w, &screen);
                bd.draw()?;
                self.apply(move_result);
            } else {
                // we're here also after end of game, when current_player is None
                let event = dam.next_event().unwrap();
                dam.unblock();
                match event {
                    Event::Key(KeyEvent { code, .. }) => {
                        let next_state = self.handle_key_event(code);
                        if let Some(next_state) = next_state {
                            return Ok(next_state);
                        }
                    }
                    Event::Resize(width, height) => {
                        screen.set_terminal_size(width, height);
                    }
                    Event::Click(x, y) => {
                        let sp = ScreenPos{ x, y };
                        let pos_converter = PosConverter::from(self.board.lapin_pos(), &screen);
                        debug!("click in {:?}", pos_converter.to_real(sp));
                    }
                    _ => {
                        debug!("ignored event: {:?}", event);
                    }
                }
            }
        }
    }
}



