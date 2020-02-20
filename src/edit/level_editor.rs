use {
    anyhow::Result,
    crate::{
        app_state::StateTransition,
        board::*,
        board_drawer::BoardDrawer,
        io::W,
        level::Level,
        play::PlayLevelState,
        pos::*,
        screen::Screen,
        status::Status,
        task_sync::*,
    },
    crossterm::{
        event::{
            KeyCode,
            KeyEvent,
        },
    },
    std::{
        boxed::Box,
        fs,
        path::PathBuf,
    },
    super::{
        LAYOUT,
        pen::Pen,
        selector::SelectorPanel,
        EditLevelState,
    },
    termimad::{
        Event,
    },
};

pub struct LevelEditor {
    board: Board,
    pub pen: Pen,
    path: PathBuf,
    status: Status,
    center: Pos,    // the pos shown at center of the screen
}

impl LevelEditor {

    pub fn new(
        state: &EditLevelState,
    ) -> Self {
        let path = state.path.to_path_buf();
        let board = Board::from(&*state.level);
        let status = Status::from_message(
            "click at random to do random things, *q* to quit, *s* to save, *t* to test".to_string()
        );
        let pen = Pen::default();
        let center = board.lapin_pos();
        Self {
            board,
            pen,
            path,
            status,
            center,
        }
    }

    fn write_status(
        &mut self,
        w: &mut W,
        screen: &Screen,
    ) -> Result<()> {
        self.status.display(w, screen)
    }

    fn save_to_file(
        &mut self,
    ) -> Result<()> {
        let level = Level::from(&self.board);
        let serialized = serde_json::to_string(&level)?;
        fs::write(&self.path, serialized)?;
        Ok(())
    }

    fn handle_key_event(
        &mut self,
        code: KeyCode,
    ) -> Option<StateTransition> {
        debug!("code: {:?}", code);
        match code {
            KeyCode::Char('q') => {
                Some(StateTransition::Quit)
            }
            KeyCode::Up => {
                self.center.y -= 1;
                None
            }
            KeyCode::Right => {
                self.center.x += 1;
                None
            }
            KeyCode::Down => {
                self.center.y += 1;
                None
            }
            KeyCode::Left => {
                self.center.x -= 1;
                None
            }
            KeyCode::Char('s') => {
                match self.save_to_file() {
                    Err(e) => {
                        self.status = Status::from_error(format!(
                            "level saving failed: `{:?}`",
                            e,
                        ));
                    }
                    _ => {
                        self.status = Status::from_message(format!(
                            "level saved in file `{:?}`",
                            &self.path,
                        ));
                    }
                }
                None
            }
            KeyCode::Char('t') => {
                Some(StateTransition::PlayLevel(PlayLevelState {
                    comes_from_edit: true,
                    path: Some(self.path.clone()),
                    level: Box::new(Level::from(&self.board)),
                }))
            }
            _ => None,
        }
    }

    /// return the next state
    pub fn run(
        &mut self,
        w: &mut W,
        dam: &mut Dam,
    ) -> Result<StateTransition> {
        let mut screen = Screen::new(LAYOUT);
        self.write_status(w, &screen)?;
        loop {
            let mut bd = BoardDrawer::new_around(&self.board, w, &screen, self.center);
            bd.draw()?;
            let mut selector = SelectorPanel::new(w, &mut self.pen, &screen);
            selector.draw()?;
            let event = dam.next_event().unwrap();
            dam.unblock();
            let next_state = match event {
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
                    debug!("click in {:?}", sp);
                    if sp.is_in(&screen.areas.board) {
                        let pos_converter = PosConverter::from(self.center, &screen);
                        let pos = pos_converter.to_real(sp);
                        debug!("click in board {:?}", pos);
                        self.pen.click(pos, &mut self.board);
                    } else if sp.is_in(&screen.areas.selector) {
                        selector.click(sp);
                    }
                    None
                }
                _ => {
                    debug!("ignored event: {:?}", event);
                    None
                }
            };
            if let Some(next_state) = next_state {
                return Ok(next_state);
            }
        }
    }
}

