
use {
    crate::{
        pos::*,
    },
    super::*,
};

#[derive(Debug, Clone, Copy)]
pub enum Action {
    Moves(Dir),
    Aims(Dir),
    StopsAiming,
    Eats(Dir, ActorId),
    Fires(Dir, ActorId),
}

#[derive(Debug, Clone, Copy)]
pub struct ActorMove {
    pub actor_id: ActorId,
    pub action: Action,
}

impl ActorMove {
    pub fn new(actor_id: ActorId, action: Action) -> Self {
        Self { actor_id, action }
    }
    pub fn target_id(self) -> Option<ActorId> {
        match self.action {
            Action::Eats(_, id) | Action::Fires(_,id) => Some(id),
            _ => None,
        }
    }
}

/// what the world plays in a non-player turn.
/// Arrays here must be consistent with the board.
#[derive(Debug)]
pub struct WorldMove {
    pub actor_moves: Vec<ActorMove>,
}
