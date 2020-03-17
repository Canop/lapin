use {
    anyhow::{
        self,
        Result,
    },
    crate::{
        app::{
            Context,
            EditCommand,
            State,
            StateTransition,
        },
        core::*,
        display::*,
        persist::{
            self,
            Level,
            Bag,
            SerdeFormat,
        },
        pos::*,
    },
    crossterm::{
        event::{
            KeyCode,
            KeyEvent,
            KeyModifiers,
        },
    },
    std::{
        convert::TryFrom,
        io::Write,
        path::PathBuf,
    },
    super::{
        LAYOUT,
        drawing_history::DrawingHistory,
        pen::Pen,
        pen_panel::PenPanel,
        head_panel::EditorHeadPanel,
    },
    termimad::{
        Event,
    },
};

const DEFAULT_STATUS: &str = "Use arrows to move, *q* to quit, *s* to save, *t* to test, *u*/*r* to undo/redo";

pub struct LevelEditor {
    board: Board,
    pub pen: Pen,
    path: PathBuf,
    status: Status,
    center: Pos,    // the pos shown at center of the screen
    history: DrawingHistory,
    head_panel: EditorHeadPanel,
    output_format: Option<SerdeFormat>,
}

impl TryFrom<&EditCommand> for LevelEditor {
    type Error = anyhow::Error;
    fn try_from(ec: &EditCommand) -> Result<Self> {
        debug!("opening level editor on {:?}", &ec.path);
        let level = if ec.path.exists() {
            let mut bag: Bag = persist::read_file(&ec.path)?;
            if let Some(level) = bag.as_sole_level() {
                level
            } else {
                return Err(anyhow!("Only single level files can be edited"));
            }
        } else {
            debug!("non existing file : starting with a clean board");
            Level::default()
        };
        let output_format = ec.output_format.as_ref()
            .and_then(|key| persist::SerdeFormat::from_key(&key));
        Ok(LevelEditor::new(
            ec.path.to_path_buf(),
            level,
            output_format,
        ))
    }
}

impl LevelEditor {

    pub fn new(
        path: PathBuf,
        level: Level,
        output_format: Option<SerdeFormat>,
    ) -> Self {
        let board = Board::from(&level);
        let status = Status::from_message(DEFAULT_STATUS.to_string());
        let pen = Pen::new_for(&level);
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
            output_format,
        }
    }

    fn save_to_file(
        &mut self,
    ) -> Result<()> {
        let level = Level::from(&self.board);
        let bag = persist::Bag::from(level);
        persist::write_file(
            &bag,
            &self.path,
            self.output_format,
            false,
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
            KeyCode::Char('c') => {
                self.center = self.board.lapin_pos();
                None
            }
            KeyCode::Char('q') => {
                Some(StateTransition::Quit)
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
            KeyCode::Char('t') => Some(StateTransition::PlayLevel{ level_idx: 0 }),
            KeyCode::Char('u') => {
                self.history.undo(&mut self.board);
                None
            }
            _ => None,
        }
    }

}

impl State for LevelEditor {

    fn label(&self) -> &'static str {
        super::LABEL
    }

    fn run(
        &mut self,
        con: &mut Context,
    ) -> Result<StateTransition> {
        let mut screen = Screen::new(LAYOUT);
        loop {
            let mut bd = BoardDrawer::new(&self.board, &screen, self.center);
            bd.draw(con)?;
            let mut pen_panel = PenPanel::new(&mut self.pen, &screen);
            pen_panel.draw(con)?;
            self.head_panel.draw(con, &self.board, &screen)?;
            self.status.display(con, &screen)?;
            con.w.flush()?;
            let event = con.dam.next_event().unwrap();
            con.dam.unblock();
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

    fn get_level(
        &self,
        _level_idx: usize,
    ) -> Option<Level> {
        Some(Level::from(&self.board))
    }

}
