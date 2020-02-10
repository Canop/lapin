use {
    crate::{
        app::AppState,
        board::*,
        command::Command,
        draw_board::BoardDrawer,
        io::W,
        screen::Screen,
        test_level,
        world,
    },
    anyhow::Result,
    crossterm::{
        cursor,
        event::{
            self,
            KeyEvent,
        },
        style::{
            Attribute,
            Color,
            ContentStyle,
            PrintStyledContent,
        },
        QueueableCommand,
    },
    std::io::Write,
    termimad::{
        Event,
        EventSource,
        gray,
    },
};

pub struct GameRunner {
    pub board: Board,
}

impl GameRunner {
    pub fn new() -> Self {
        let board = test_level::build();
        Self {
            board,
        }
    }

    fn handle_event(
        &mut self,
        w: &mut W,
        event: Event,
    ) -> Result<Option<AppState>> {
        let screen = Screen::new()?;
        Ok(match event {
            Event::Key(KeyEvent { code, .. }) => {
                debug!("key code : {:?}", code);
                match Command::from(code) {
                    None => None,
                    Some(Command::Quit) => {
                        Some(AppState::Quit)
                    }
                    Some(cmd) => {
                        debug!("cmd: {:?}", &cmd);
                        let move_result = self.board.apply_player_move(cmd);
                        let mut bd = BoardDrawer::new(&self.board, w, &screen);
                        bd.draw()?;
                        match move_result {
                            MoveResult::Ok => {
                                let world_move = world::play(&self.board);
                                debug!("world_move: {:?}", &world_move);
                                bd.animate(&world_move)?;
                                let move_result = self.board.apply_world_move(world_move);
                                next_state(move_result)
                            }
                            _ => next_state(move_result)
                        }
                    }
                }
            }
            _ => {
                debug!("ignored event: {:?}", event);
                None
            }
        })
    }

    /// return the next state
    pub fn run(
        &mut self,
        w: &mut W,
    ) -> Result<AppState> {
        let screen = Screen::new()?;
        let cs = ContentStyle {
            foreground_color: Some(gray(15)),
            background_color: None,
            attributes: Attribute::Bold.into(),
        };
        w.queue(cursor::MoveTo(10, screen.height-1))?;
        w.queue(PrintStyledContent(cs.apply("hit arrows to move, 'q' to quit".to_string())))?;
        let event_source = EventSource::new()?;
        let rx_events = event_source.receiver();
        let mut next_state = None;
        loop {
            let mut bd = BoardDrawer::new(&self.board, w, &screen);
            bd.draw()?;
            w.flush()?;
            let e = rx_events.recv();
            debug!(" {:?} <--", e);
            match e {
                Ok(event) => {
                    next_state = self.handle_event(w, event)?;
                }
                Err(_) => {
                    debug!("channel err -> normal end");
                    break; // normal end of application
                }
            }
            debug!("event handled");
            event_source.unblock(next_state.is_some());
            debug!("unblocked");
        }
        match next_state {
            Some(state) => Ok(state), // normal
            None => {
                debug!("unexpected lack of next state!");
                Ok(AppState::Quit)
            }
        }
    }
}

fn next_state(move_result: MoveResult) -> Option<AppState> {
    match move_result {
        MoveResult::PlayerWin => {
            Some(AppState::Message("You WIN!".to_string()))
        }
        MoveResult::PlayerLose => {
            Some(AppState::Message("You LOSE!".to_string()))
        }
        _ => None,
    }
}


