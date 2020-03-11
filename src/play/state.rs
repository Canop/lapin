use {
    anyhow::Result,
    crate::{
        app::{
            Context,
            State,
            StateTransition,
        },
        core::*,
        display::{
            BoardDrawer,
            Screen,
            Status,
        },
        edit,
        persist::Level,
        pos::*,
        win_db::{
            self,
            Signature,
        },
    },
    crossterm::{
        event::{
            KeyCode,
            KeyEvent,
            KeyModifiers,
        },
    },
    std::{
        time::SystemTime,
    },
    super::LAYOUT,
    termimad::{
        Event,
    },
};

pub struct PlayLevelState {
    level_signature: Signature,
    comes_from_editor: bool,
    board: Board,
    center: Pos,    // the pos shown at center of the screen
    status: Status,
}

impl PlayLevelState {

    /// create a new game state. When coming from editor
    /// the win is not saved
    pub fn new(
        level: &Level,
        previous_state: Option<&'static str>,
    ) -> Result<Self> {
        let board = Board::from(level);
        let status = Status::from_message(
            if let Some(state) = previous_state {
                format!(
                    "Hit *arrows* to move, *?* for help, *esc* to go back to {}, *q* to quit",
                    state,
                )
            } else {
                "Hit *arrows* to move, *?* for help, *q* to quit".to_string()
            }
        );
        let level_signature = Signature::new(level)?;
        let comes_from_editor = previous_state == Some(edit::LABEL);
        let center = board.lapin_pos();
        Ok(Self {
            level_signature,
            comes_from_editor,
            board,
            center,
            status,
        })
    }

    fn handle_player_dir(
        &mut self,
        dir: Dir,
    ) -> Option<StateTransition> {
        let move_result = self.board.apply_player_move(dir);
        self.apply(move_result);
        self.center = self.board.lapin_pos();
        None
    }

    fn handle_screen_dir(
        &mut self,
        dir: Dir,
    ) -> Option<StateTransition> {
        self.center = self.center.in_dir(dir);
        None
    }

    fn center_on_lapin(&mut self) -> Option<StateTransition> {
        self.center = self.board.lapin_pos();
        None
    }

    fn handle_key_event(
        &mut self,
        key_event: KeyEvent,
    ) -> Result<Option<StateTransition>> {
        Ok(match (
            self.comes_from_editor,
            key_event.modifiers.contains(KeyModifiers::CONTROL),
            key_event.code,
        ) {
            (_, false, KeyCode::Esc) => Some(StateTransition::Back),
            (_, false, KeyCode::Up) => self.handle_player_dir(Dir::Up),
            (_, false, KeyCode::Right) => self.handle_player_dir(Dir::Right),
            (_, false, KeyCode::Down) => self.handle_player_dir(Dir::Down),
            (_, false, KeyCode::Left) => self.handle_player_dir(Dir::Left),
            (_, false, KeyCode::Char('q')) => Some(StateTransition::Quit),
            (_, false, KeyCode::Char('?')) => Some(StateTransition::Help),
            (true, false, KeyCode::Char('c')) => self.center_on_lapin(),
            (true, true, KeyCode::Up) => self.handle_screen_dir(Dir::Up),
            (true, true, KeyCode::Right) => self.handle_screen_dir(Dir::Right),
            (true, true, KeyCode::Down) => self.handle_screen_dir(Dir::Down),
            (true, true, KeyCode::Left) => self.handle_screen_dir(Dir::Left),
            _ => None,
        })
    }

    fn set_end_status(&mut self, reason: &str, win: bool) {
        self.status = Status::from(
            format!(
                "{} You **{}!** - hit *q* to quit, *esc* to go back to {}",
                reason,
                if win { "WIN" } else { "LOSE" },
                if self.comes_from_editor { "editor" } else { "home" },
            ),
            !win
        );
    }

    /// change the state accordingly to the move_result
    /// returned on a move by either the player or the world
    fn apply(&mut self, move_result: MoveResult) {
        match move_result {
            MoveResult::PlayerWin(s) => {
                self.set_end_status(&s, true);
                if !self.comes_from_editor {
                    if let Err(e) = win_db::save_win(&self.level_signature) {
                        warn!("Saving win failed: {:?}", e);
                    }
                }
            }
            MoveResult::PlayerLose(s) => {
                self.set_end_status(&s, false);
            }
            _ => {}
        }
    }

    fn write_status(
        &mut self,
        con: &mut Context,
        screen: &Screen,
    ) -> Result<()> {
        self.status.display(con, screen)
    }
}

impl State for PlayLevelState {

    fn label(&self) -> &'static str {
        "game"
    }

    fn run(
        &mut self,
        con: &mut Context,
    ) -> Result<StateTransition> {
        let mut screen = Screen::new(LAYOUT);
        let mut seed = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .map_or(0, |d| (d.as_secs()%7) as usize);
        loop {
            BoardDrawer::new(&self.board, &screen, self.center).draw(con)?;
            self.write_status(con, &screen)?;
            if self.board.current_player == Player::World {
                let world_player = WorldPlayer::new(&self.board, seed);
                seed += 1;
                let mut world_move = time!(Info, "world play", world_player.play());
                let actors = self.board.actors.clone();
                let move_result = self.board.apply_world_move(&mut world_move);
                let mut bd = BoardDrawer::new(&self.board, &screen, self.center);
                bd.animate(con, &actors, &world_move)?;
                bd.draw(con)?;
                self.apply(move_result);
            } else {
                // we're here also after end of game, when current_player is None
                let event = con.dam.next_event().unwrap();
                con.dam.unblock();
                match event {
                    Event::Key(ke) => {
                        let next_state = self.handle_key_event(ke)?;
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

    // this one isn't used now but could be if we offer to jump from any level
    // to edition
    fn get_level(
        &self,
        _level_idx: usize,
    ) -> Option<Level> {
        Some(Level::from(&self.board))
    }

}

