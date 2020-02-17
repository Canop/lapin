use {
    anyhow::Result,
    crate::{
        app::AppState,
        board::*,
        consts::*,
        draw_board::BoardDrawer,
        fromage::EditSubCommand,
        io::W,
        level::Level,
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
        fs::{
            self,
            File,
        },
        io::{
            BufReader,
        },
        path::PathBuf,
    },
    super::{
        pen::Pen,
    },
    termimad::{
        Event,
    },
};


pub struct LevelEditor {
    board: Board,
    pen: Pen,
    path: PathBuf,
    status: Status,
}

impl LevelEditor {

    pub fn new(esc: EditSubCommand) -> Result<Self> {
        let path = esc.path;
        debug!("opening level editor on {:?}", &path);
        let board = if path.exists() {
            debug!("trying to deserialize the file");
            let file = File::open(&path)?;
            let level: Level = serde_json::from_reader(file)?;
            // FIXME call validity checks here
            Board::from(&level)
        } else {
            debug!("non existing file : starting with a clean board");
            Board::new(
                PosArea::new(-100..100, -100..100),
                FIELD,
            )
        };
        let status = Status::from_message(
            "click at random to do random things, *q* to quit, *s* to save".to_string()
        );
        let pen = Pen::default();
        Ok(Self {
            board,
            pen,
            path,
            status,
        })
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
    ) -> Option<AppState> {
        debug!("code: {:?}", code);
        match code {
            KeyCode::Char('q') => {
                Some(AppState::Quit)
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
            _ => None,
        }
    }

    /// return the next state
    pub fn run(
        &mut self,
        w: &mut W,
        dam: &mut Dam,
    ) -> Result<AppState> {
        let mut screen = Screen::new()?;
        self.write_status(w, &screen)?;
        loop {
            let mut bd = BoardDrawer::new(&self.board, w, &screen);
            bd.draw()?;
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
                    let pos = bd.pos_converter.to_real(sp);
                    debug!("click in {:?}", pos);
                    self.pen.click(pos, &mut self.board);
                    None
                }
                _ => {
                    debug!("ignored event: {:?}", event);
                    None
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

