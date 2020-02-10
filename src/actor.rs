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
pub enum Kind {
    Lapin, // there's only one
    Knight,
    Fox,
}
impl Kind {
    pub fn eats(self, other: Kind) -> bool {
        match (self, other) {
            (Kind::Knight, Kind::Fox) => true,
            (Kind::Fox, Kind::Lapin) => true,
            _ => false,
        }
    }
    pub fn skin(self, skin: &Skin) -> FgSkin {
        match self {
            Kind::Lapin => skin.lapin,
            Kind::Knight => skin.knight,
            Kind::Fox => skin.fox,
        }
    }
}

// an actor id is only temporarly valid
//put type ActorId = usize;

#[derive(Debug, Clone, Copy)]
pub struct Actor {
    pub kind: Kind,
    pub pos: Pos,
    pub state: ActorState,
}
impl Actor {
    pub fn eats(self, other: Actor) -> bool {
        self.kind.eats(other.kind)
    }
    pub fn new(kind: Kind, x: Int, y: Int) -> Self {
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

