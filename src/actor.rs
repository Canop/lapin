use {
    crate::{
        path::*,
        pos::*,
        skin::*,
    },
    serde::{Serialize, Deserialize},
};

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub struct ActorState {
    pub aim: Option<Dir>,
    pub drunk: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ActorKind {
    Lapin,
    Knight,
    Wolf,
    Fox,
    Hunter,
    Sheep,
}
pub static ACTORS: &[ActorKind] = &[
    ActorKind::Lapin,
    ActorKind::Knight,
    ActorKind::Wolf,
    ActorKind::Fox,
    ActorKind::Hunter,
    ActorKind::Sheep,
];
impl ActorKind {
    pub fn drinks_wine(self) -> bool {
        match self {
            ActorKind::Hunter => true,
            _ => false,
        }
    }
    pub fn hits(self, other: Self) -> bool {
        use ActorKind::*;
        match (self, other) {
            (Fox, Lapin) => true,
            (Knight, Fox) => true,
            (Knight, Hunter) => true,
            (Knight, Wolf) => true,
            (Wolf, Hunter) => true,
            (Wolf, Sheep) => true,
            (Wolf, Lapin) => true,
            _ => false,
        }
    }
    pub fn runs_after(self, other: Self) -> bool {
        use ActorKind::*;
        match (self, other) {
            (Fox, Lapin) => true,
            (Hunter, Lapin) => true,
            (Knight, Fox) => true,
            (Wolf, Hunter) => true,
            (Wolf, Sheep) => true,
            (Wolf, Lapin) => true,
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
            Fox => skin.fox,
            Hunter => skin.hunter,
            Knight => skin.knight,
            Lapin => skin.lapin,
            Sheep => skin.sheep,
            Wolf => skin.wolf,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Actor {
    pub kind: ActorKind,
    pub pos: Pos, // TODO remove and use Located<Actor> where the pos is needed?
    pub state: ActorState,
}
impl Actor {
    pub fn new(kind: ActorKind, x: Int, y: Int) -> Self {
        Self {
            kind,
            pos: Pos::new(x, y),
            state: ActorState::default(),
        }
    }
    pub fn hits(self, other: Actor) -> bool {
        self.kind.hits(other.kind)
    }
    pub fn fires_on(self, other: Actor) -> bool {
        use ActorKind::*;
        match self.kind {
            Hunter => if self.state.drunk {
                true
            } else {
                match other.kind {
                    Fox => true,
                    Knight => true,
                    Lapin => true,
                    Wolf => true,
                    _ => false,
                }
            }
            _ => false,
        }
    }
    pub fn runs_after(self, other: Actor) -> bool {
        self.kind.runs_after(other.kind)
    }
    pub fn is_aiming(self) -> bool {
        self.state.aim.is_some()
    }
    pub fn path_finding_strategy(self) -> PathFindingStrategy {
        use {
            ActorKind::*,
            PathFindingStrategy::*,
        };
        match self.kind {
            Hunter if self.state.drunk => Quadrant,
            Sheep => BestToNearest, // sheeps are kind of stupid
            _ => Best,
        }
    }
    pub fn skin(self, skin: &Skin) -> FgSkin {
        let mut s = self.kind.skin(skin);
        if let Some(dir) = self.state.aim {
            s.chr = match dir {
                Dir::Up => skin.aiming_up,
                Dir::Right => skin.aiming_right,
                Dir::Down => skin.aiming_down,
                Dir::Left => skin.aiming_left,
                _ => '?',
            };
        }
        if self.state.drunk {
            s.color = skin.hunter_drunk.color;
        }
        s
    }
}


// note that it's possible to insert an actor at
// a position other than its one (it can be for example
// his target) using `set`
pub type ActorPosMap = OptionPosMap<Actor>;
impl ActorPosMap {
    pub fn from(area: PosArea) -> Self {
        PosMap::<Option<Actor>>::new(area, None)
    }
    pub fn insert(&mut self, actor: Actor) {
        self.set(actor.pos, Some(actor));
    }
}
