use {
    crate::{
        app::AppState,
        board::*,
        command::Command,
        consts::*,
        draw_board::BoardDrawer,
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
}

impl LevelEditor {
    pub fn new() -> Self {
        let board = Board::new(
            PosArea::new(-100..100, -100..100),
            FIELD,
        );
        let pen = Pen::default();
        Self {
            board,
            pen,
        }
    }

    fn write_status(
        &mut self,
        w: &mut W,
        screen: &Screen,
    ) -> Result<()> {
        Status::from_message(mad_inline!(
            "click at random to do random things, *q* to quit"
        ))
        .display(w, screen)
    }

    fn handle_key_event(
        &mut self,
        code: KeyCode,
    ) -> Option<AppState> {
        match Command::from(code) {
            Some(Command::Quit) => {
                Some(AppState::Quit)
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

