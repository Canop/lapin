use {
    crate::{
        pos::*,
    },
    crossterm::{
        event::{KeyCode},
    },
};

#[derive(Debug, Clone, Copy)]
pub enum Command {
    Move(Dir),
    Quit,
}

impl Command {
    // provisoire, plus tard un accumulateur permettra de combiner des touches
    pub fn from(key_code: KeyCode) -> Option<Self> {
        match key_code {
            KeyCode::Up => Some(Command::Move(Dir::Up)),
            KeyCode::Right => Some(Command::Move(Dir::Right)),
            KeyCode::Down => Some(Command::Move(Dir::Down)),
            KeyCode::Left => Some(Command::Move(Dir::Left)),
            KeyCode::Char('q') => Some(Command::Quit),
            _ => None,
        }
    }
}
