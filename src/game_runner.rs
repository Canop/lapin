use {
    crate::{
        app::AppState,
        board::*,
        command::Command,
        draw_board::BoardDrawer,
        io::W,
        pos::*,
        screen::Screen,
        task_sync::*,
        test_level,
        world::*,
    },
    anyhow::Result,
    crossterm::{
        cursor,
        event::{
            KeyEvent,
        },
        style::{
            Attribute,
            ContentStyle,
            PrintStyledContent,
        },
        QueueableCommand,
    },
    termimad::{
        Event,
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
        event: Event,
        pos_converter: PosConverter,
    ) -> Result<Option<AppState>> {
        Ok(match event {
            Event::Key(KeyEvent { code, .. }) => {
                match Command::from(code) {
                    None => None,
                    Some(Command::Quit) => {
                        Some(AppState::Quit)
                    }
                    Some(cmd) => {
                        let move_result = self.board.apply_player_move(cmd);
                        next_state(move_result)
                    }
                }
            }
            Event::Click(x, y) => {
                let sp = ScreenPos{ x, y };
                debug!("click in {:?}", pos_converter.to_real(sp));
                None
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
        dam: &mut Dam,
    ) -> Result<AppState> {
        let screen = Screen::new()?;
        let cs = ContentStyle {
            foreground_color: Some(gray(15)),
            background_color: None,
            attributes: Attribute::Bold.into(),
        };
        w.queue(cursor::MoveTo(10, screen.height-1))?;
        w.queue(PrintStyledContent(cs.apply("hit arrows to move, 'q' to quit".to_string())))?;
        loop {
            let mut bd = BoardDrawer::new(&self.board, w, &screen);
            let pos_converter = bd.pos_converter;
            bd.draw()?;
            let next_state = match self.board.current_player {
                Player::Lapin => {
                    let event = dam.next_event().unwrap();
                    dam.unblock();
                    self.handle_event(event, pos_converter)?
                }
                Player::World => {
                    let world_player = WorldPlayer::new(&self.board);
                    let world_move = world_player.play();
                    bd.animate(&world_move, dam)?;
                    let move_result = self.board.apply_world_move(world_move);
                    next_state(move_result)
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


