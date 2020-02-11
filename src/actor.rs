use {
    crate::{
        pos::*,
        skin::*,
    },
};

// will be backed by a bit seat as soon as there's
// more content
#[derive(Debug, Clone, Copy)]
pub struct ActorState {
}
impl ActorState {
    pub fn new() -> Self {
        Self { }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ActorKind {
    Lapin, // there's only one
    Knight,
    Fox,
}
impl ActorKind {
    pub fn eats(self, other: ActorKind) -> bool {
        match (self, other) {
            (ActorKind::Knight, ActorKind::Fox) => true,
            (ActorKind::Fox, ActorKind::Lapin) => true,
            _ => false,
        }
    }
    pub fn skin(self, skin: &Skin) -> FgSkin {
        match self {
            ActorKind::Lapin => skin.lapin,
            ActorKind::Knight => skin.knight,
            ActorKind::Fox => skin.fox,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Actor {
    pub kind: ActorKind,
    pub pos: Pos,
    pub state: ActorState,
}
impl Actor {
    pub fn eats(self, other: Actor) -> bool {
        self.kind.eats(other.kind)
    }
    pub fn new(kind: ActorKind, x: Int, y: Int) -> Self {
        Self {
            kind,
            pos: Pos::new(x, y),
            state: ActorState::new(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ActorMove {
    pub actor_id: usize,
    pub target_id: Option<usize>, // the actor killed by the move
    pub dir: Dir,
}

