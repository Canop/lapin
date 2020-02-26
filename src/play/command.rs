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
    Back,
    Move(Dir),
    Help,
    Quit,
}

impl Command {
    pub fn from(key_code: KeyCode) -> Option<Self> {
        match key_code {
            KeyCode::Esc => Some(Command::Back),
            KeyCode::Up => Some(Command::Move(Dir::Up)),
            KeyCode::Right => Some(Command::Move(Dir::Right)),
            KeyCode::Down => Some(Command::Move(Dir::Down)),
            KeyCode::Left => Some(Command::Move(Dir::Left)),
            KeyCode::Char('q') => Some(Command::Quit),
            KeyCode::Char('?') => Some(Command::Help),
            _ => None,
        }
    }
}
