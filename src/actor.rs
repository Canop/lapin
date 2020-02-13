use {
    crate::{
        pos::*,
        skin::*,
    },
};

// will be backed by a bit seat as soon as there's
// more content
#[derive(Debug, Clone, Copy)]
pub enum ActorState {
    Normal,
    Aiming(Dir),
}

#[derive(Debug, Clone, Copy)]
pub enum ActorKind {
    Lapin,
    Knight,
    Wolf,
    Fox,
    Hunter,
}
impl ActorKind {
    pub fn hits(self, other: Self) -> bool {
        use ActorKind::*;
        match (self, other) {
            (Knight, Fox) => true,
            (Knight, Hunter) => true,
            (Knight, Wolf) => true,
            (Wolf, Hunter) => true,
            (Wolf, Lapin) => true,
            (Fox, Lapin) => true,
            _ => false,
        }
    }
    pub fn runs_after(self, other: Self) -> bool {
        use ActorKind::*;
        match (self, other) {
            (Knight, Fox) => true,
            //(Knight, Hunter) => true,
            (Wolf, Hunter) => true,
            (Wolf, Lapin) => true,
            (Fox, Lapin) => true,
            (Hunter, Lapin) => true,
            _ => false,
        }
    }
    pub fn fires_on(self, _other: Self) -> bool {
        match self {
            Self::Hunter => true,
            _ => false,
        }
    }
    pub fn is_immune_to_fire(self) -> bool {
        match self {
            Self::Knight => true,
            _ => false,
        }
    }
    pub fn skin(self, skin: &Skin) -> FgSkin {
        use ActorKind::*;
        match self {
            Lapin => skin.lapin,
            Knight => skin.knight,
            Wolf => skin.wolf,
            Fox => skin.fox,
            Hunter => skin.hunter,
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
    pub fn hits(self, other: Actor) -> bool {
        self.kind.hits(other.kind)
    }
    pub fn fires_on(self, other: Actor) -> bool {
        self.kind.fires_on(other.kind)
    }
    pub fn runs_after(self, other: Actor) -> bool {
        self.kind.runs_after(other.kind)
    }
    pub fn new(kind: ActorKind, x: Int, y: Int) -> Self {
        Self {
            kind,
            pos: Pos::new(x, y),
            state: ActorState::Normal,
        }
    }
    pub fn is_aiming(self) -> bool {
        match self.state {
            ActorState::Aiming(_) => true,
            _ => false,
        }
    }
    pub fn skin(self, skin: &Skin) -> FgSkin {
        let mut s = self.kind.skin(skin);
        if let ActorState::Aiming(dir) = self.state {
            s.chr = match dir {
                Dir::Up => skin.aiming_up,
                Dir::Right => skin.aiming_right,
                Dir::Down => skin.aiming_down,
                Dir::Left => skin.aiming_left,
                _ => '?',
            };
        }
        s
    }
}

