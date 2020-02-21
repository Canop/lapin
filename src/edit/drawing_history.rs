
use {
    crate::{
        board::*,
        level::Level,
    },
    super::{
        drawing_action::*,
    },
};


#[derive(Debug)]
pub struct DrawingHistory<'l> {
    actions: Vec<DrawingAction>,
    cursor: usize, // is actions.len() when not in redo
    initial_state: &'l Level,
}

impl<'l> DrawingHistory<'l> {
    pub fn new(level: &'l Level) -> Self {
        Self {
            actions: Vec::new(),
            cursor: 0,
            initial_state: level,
        }
    }
    pub fn can_redo(&self) -> bool {
        self.cursor < self.actions.len()
    }
    /// add the action to history and changes the board. If we're in
    /// an undo state, the future history is lost.
    pub fn apply(&mut self, action: DrawingAction, board: &mut Board) {
        if self.can_redo() {
            self.actions.truncate(self.cursor);
        }
        action.apply_to(board);
        self.actions.push(action);
        self.cursor = self.actions.len();
    }
    /// revert to the state before the current action.
    /// Return false if nothing changed (i.e. we're already at
    /// the initial state)
    pub fn undo(&mut self, board: &mut Board) -> bool {
        if self.cursor == 0 {
            return false;
        }
        self.cursor -= 1;
        board.reset_to(self.initial_state);
        for i in 0..self.cursor {
            self.actions[i].apply_to(board);
        }
        true
    }
    /// cancel one undo, if possible
    pub fn redo(&mut self, board: &mut Board) -> bool {
        if !self.can_redo() {
            return false;
        }
        self.actions[self.cursor].apply_to(board);
        self.cursor += 1;
        true
    }
}


