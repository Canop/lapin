use {
    crate::{
        board::Board,
        draw_board::BoardDrawer,
        command::Command,
        consts::*,
        io::W,
        screen::Screen,
        world,
    },
    anyhow::Result,
    crossterm::{
        event::{self, Event, KeyCode::*, KeyEvent},
    },
    std::io::Write,
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
        for y in 0..12 {
            board.set_at(46, y, WATER);
        }
        board.lapin.pos.x = 30;
        let screen = Screen::new()?;
        Ok(Self {
            board,
            screen,
        })
    }
    pub fn write(&self, w: &mut W) -> Result<()> {
        //let style = CompoundStyle::new(Some(Color::Blue), None, Attributes::default());
        //style.queue_str(w, "Lapin!")?;
        //self.board.draw(w, &self.screen)?;
        let mut bd = BoardDrawer::new(&self.board, w, &self.screen);
        bd.draw()?;
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
                    gr.board.apply_player_move(cmd);
                    let world_move = world::play(&gr.board);
                    debug!("world_move: {:?}", &world_move);
                    gr.board.apply_world_move(world_move);
                }
            }
        }
    }
    Ok(())
}
