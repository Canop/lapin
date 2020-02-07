use {
    crate::{
        board::Board,
        command::Command,
        consts::*,
        io::W,
        screen::Screen,
    },
    anyhow::Result,
    crossterm::{
        event::{self, Event, KeyCode::*, KeyEvent},
        style::{Attributes, Color},
    },
    std::io::Write,
    termimad::{CompoundStyle},
};

pub struct GameRunner {
    pub board: Board,
    pub screen: Screen,
}

impl GameRunner {
    pub fn new() -> Result<Self> {
        let mut board = Board::new(60, 20);
        board.set_borders(WALL);
        board.set_at(2, 3, WALL);
        for x in 6..17 {
            board.set_at(x, 4, WALL);
        }
        board.set_at(6, 5, WALL);
        for x in 5..11 {
            board.set_at(x, 0, FOREST);
        }
        board.lapin.pos.x = 30;
        let screen = Screen::new()?;
        Ok(Self {
            board,
            screen,
        })
    }
    pub fn write(&self, w: &mut W) -> Result<()> {
        let style = CompoundStyle::new(Some(Color::Blue), None, Attributes::default());
        style.queue_str(w, "Lapin!")?;
        self.board.draw(w, &self.screen)?;
        Ok(())
    }
}

pub fn run(w: &mut W) -> Result<()> {
    let mut gr = GameRunner::new()?;
    loop {
        gr.write(w)?;
        w.flush()?;
        if let Ok(Event::Key(KeyEvent { code, .. })) = event::read() {
            match Command::from(code) {
                None => { }
                Some(Command::Quit) => break,
                Some(cmd) => {
                    gr.board.apply(cmd);
                }
            }
        }
    }
    Ok(())
}
