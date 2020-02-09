use {
    crate::{
        app::AppState,
        board::*,
        draw_board::BoardDrawer,
        command::Command,
        consts::*,
        io::W,
        screen::Screen,
        world,
    },
    anyhow::Result,
    crossterm::{
        event::{self, Event, KeyEvent},
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
        //board.set_borders(WALL);
        board.set_at(2, 3, WALL);
        for x in 6..17 {
            board.set_at(x, 4, WALL);
        }
        for x in 8..37 {
            board.set_at(x, 8, WALL);
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
        let mut bd = BoardDrawer::new(&self.board, w, &self.screen);
        bd.draw()?;
        Ok(())
    }
}

fn end_message(move_result: &MoveResult) -> Result<AppState> {
    match move_result {
        MoveResult::PlayerWin => {
            return Ok(AppState::Message("You WIN!".to_string()));
        }
        MoveResult::PlayerLose => {
            return Ok(AppState::Message("You LOSE!".to_string()));
        }
        _ => Err(anyhow!("Invalid Internal State"))

    }
}

/// return the next state
pub fn run(w: &mut W) -> Result<AppState> {
    let mut gr = GameRunner::new()?;
    loop {
        gr.write(w)?;
        w.flush()?;
        if let Ok(Event::Key(KeyEvent { code, .. })) = event::read() {
            match Command::from(code) {
                None => { }
                Some(Command::Quit) => break,
                Some(cmd) => {
                    let move_result = gr.board.apply_player_move(cmd);
                    match &move_result {
                        MoveResult::Invalid => { continue; }
                        MoveResult::Ok => {
                            let mut bd = BoardDrawer::new(&gr.board, w, &gr.screen);
                            bd.draw()?;
                            let world_move = world::play(&gr.board);
                            debug!("world_move: {:?}", &world_move);
                            bd.animate(&world_move)?;
                            let move_result = gr.board.apply_world_move(world_move);
                            match move_result {
                                MoveResult::Ok => { continue; }
                                _ => { return end_message(&move_result); }
                            }
                        }
                        _ => { return end_message(&move_result); }
                    }
                }
            }
        }
    }
    Ok(AppState::Quit)
}
