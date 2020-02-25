use {
    crate::{
        app_state::StateTransition,
        board::*,
        board_drawer::BoardDrawer,
        edit::EditLevelState,
        io::W,
        pos::*,
        screen::Screen,
        status::Status,
        task_sync::*,
    },
    anyhow::Result,
    crossterm::{
        event::{
            KeyCode,
            KeyEvent,
        },
    },
    std::{
        time::SystemTime,
    },
    super::*,
    termimad::{
        Event,
    },
};

pub struct GameRunner<'s> {
    board: Board,
    status: Status,
    state: &'s PlayLevelState, // start state
}

impl<'s> GameRunner<'s> {
    pub fn new(state: &'s PlayLevelState) -> Result<Self> {
        let board = Board::from(&*state.level);
        let status = Status::from_message(
            "Hit *arrows* to move, *q* to quit".to_string()
        );
        Ok(Self {
            board,
            status,
            state,
        })
    }

    fn handle_key_event(
        &mut self,
        code: KeyCode,
    ) -> Option<StateTransition> {
        match Command::from(code) {
            None => None,
            Some(Command::Back) if self.state.comes_from_edit => {
                if let Some(path) = &self.state.path {
                    Some(StateTransition::EditLevel(EditLevelState{
                        path: path.clone(),
                        level: self.state.level.clone(),
                    }))
                } else {
                    None
                }
            }
            Some(Command::Quit) => {
                Some(StateTransition::Quit)
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
    ) -> Result<StateTransition> {
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
                    Event::Click(x, y, ..) => {
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



