use {
    anyhow::Result,
    crate::{
        app_state::StateTransition,
        board::*,
        board_drawer::BoardDrawer,
        fromage::*,
        io::W,
        level::Level,
        play::PlayLevelState,
        pos::*,
        screen::Screen,
        serde,
        status::Status,
        task_sync::*,
    },
    crossterm::{
        event::{
            KeyCode,
            KeyEvent,
            KeyModifiers,
        },
    },
    std::{
        boxed::Box,
        path::PathBuf,
    },
    super::{
        LAYOUT,
        drawing_history::DrawingHistory,
        pen::Pen,
        pen_panel::PenPanel,
        head_panel::EditorHeadPanel,
        EditLevelState,
    },
    termimad::{
        Event,
    },
};

const DEFAULT_STATUS: &str = "Use arrows to move, *q* to quit, *s* to save, *t* to test, *u*/*r* to undo/redo";

pub struct LevelEditor<'l> {
    board: Board,
    pub pen: Pen,
    path: PathBuf,
    status: Status,
    center: Pos,    // the pos shown at center of the screen
    history: DrawingHistory<'l>,
    head_panel: EditorHeadPanel,
    fromage: &'l Fromage,
}

impl<'l> LevelEditor<'l> {

    pub fn new(
        state: &'l EditLevelState,
        fromage: &'l Fromage,
    ) -> Self {
        let level = &*state.level;
        let path = state.path.to_path_buf();
        let board = Board::from(level);
        let status = Status::from_message(DEFAULT_STATUS.to_string());
        let pen = Pen::new_for(level);
        let center = board.lapin_pos();
        let history = DrawingHistory::new(level);
        let head_panel = EditorHeadPanel::new();
        Self {
            board,
            pen,
            path,
            status,
            center,
            history,
            head_panel,
            fromage,
        }
    }

    fn save_to_file(
        &mut self,
    ) -> Result<()> {
        let level = Level::from(&self.board);
        let format = self.fromage.output_format()
            .and_then(|key| serde::SerdeFormat::from_key(&key));
        serde::write_file(
            &level,
            &self.path,
            format,
        )
    }

    fn handle_key_event(
        &mut self,
        code: KeyCode,
    ) -> Option<StateTransition> {
        debug!("code: {:?}", code);
        if self.head_panel.handle_key_event(code, &mut self.board) {
            return None; // the event was handled by the input field
        }
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
            KeyCode::Char('r') => {
                self.history.redo(&mut self.board);
                None
            }
            KeyCode::Char('s') => {
                match self.save_to_file() {
                    Err(e) => {
                        warn!("error while saving level: {:?}", e);
                        self.status = Status::from_error("level saving failed".to_string());
                    }
                    _ => {
                        self.status = Status::from_message("level saved".to_string());
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
            KeyCode::Char('u') => {
                self.history.undo(&mut self.board);
                None
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
        loop {
            let mut bd = BoardDrawer::new_around(&self.board, w, &screen, self.center);
            bd.draw()?;
            let mut pen_panel = PenPanel::new(&mut self.pen, &screen);
            pen_panel.draw(w)?;
            self.head_panel.draw(w, &self.board, &screen)?;
            self.status.display(w, &screen)?;
            let event = dam.next_event().unwrap();
            dam.unblock();
            let next_state = match event {
                Event::Key(KeyEvent { code, .. }) => {
                    self.handle_key_event(code)
                }
                Event::Resize(width, height) => {
                    screen.set_terminal_size(width, height);
                    None
                }
                Event::Click(x, y, modifiers) => {
                    let sp = ScreenPos{ x, y };
                    debug!("click in {:?}", sp);
                    let action = if sp.is_in(&screen.areas.header) {
                        self.head_panel.click(sp, &mut self.board)
                    } else {
                        self.head_panel.click_outside(&mut self.board);
                        if sp.is_in(&screen.areas.board) {
                            let pos_converter = PosConverter::from(self.center, &screen);
                            self.pen.click(
                                pos_converter.to_real(sp),
                                modifiers.contains(KeyModifiers::CONTROL),
                            )
                        } else if sp.is_in(&screen.areas.pen_panel) {
                            pen_panel.click(sp);
                            None
                        } else {
                            None
                        }
                    };
                    if let Some(action) = action {
                        self.history.apply(action, &mut self.board);
                    }
                    self.status = Status::from_message(
                        self.pen.status_help().unwrap_or(DEFAULT_STATUS.to_string())
                    );
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

