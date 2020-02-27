use {
    crate::{
        pos::*,
        skin::*,
        terrain::*,
    },
    serde::{Serialize, Deserialize},
    std::fmt,
    termimad::{
        StyledChar,
    },
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
impl fmt::Display for ActorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use ActorKind::*;
        match self {
            Lapin => write!(f, "lapin"),
            Knight => write!(f, "knight"),
            Wolf => write!(f, "wolf"),
            Fox => write!(f, "fox"),
            Hunter => write!(f, "hunter"),
            Sheep => write!(f, "sheep"),
        }
    }
}
pub static ACTORS: &[ActorKind] = &[
    ActorKind::Lapin,
    ActorKind::Knight,
    ActorKind::Wolf,
    ActorKind::Fox,
    ActorKind::Hunter,
    ActorKind::Sheep,
];
pub static FOX_PREYS: &[ActorKind] = &[
    ActorKind::Lapin,
];
pub static KNIGHT_PREYS: &[ActorKind] = &[
    ActorKind::Wolf,
    ActorKind::Fox,
    ActorKind::Hunter,
];
pub static HUNTER_PREYS: &[ActorKind] = &[ // when not drunk
    ActorKind::Lapin,
    ActorKind::Wolf,
    ActorKind::Fox,
];
pub static WOLF_PREYS: &[ActorKind] = &[
    ActorKind::Lapin,
    ActorKind::Hunter,
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
    pub fn skin(self, skin: &Skin) -> &StyledChar {
        use ActorKind::*;
        match self {
            Fox => &skin.fox,
            Hunter => &skin.hunter,
            Knight => &skin.knight,
            Lapin => &skin.lapin,
            Sheep => &skin.sheep,
            Wolf => &skin.wolf,
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
    pub fn can_enter(self, terrain: Terrain) -> bool {
        use {ActorKind::*, Terrain::*};
        match (self.kind, terrain) {
            (_, Mud) => true,
            (_, Grass) => true,
            (Knight, Sand) => false,
            (_, Sand) => true,
            _ => false, // water and wall
        }
    }
    pub fn preys(self) -> Option<&'static[ActorKind]> {
        use ActorKind::*;
        match self.kind {
            Fox => Some(FOX_PREYS),
            Knight => Some(KNIGHT_PREYS),
            Hunter => Some(HUNTER_PREYS),
            Wolf => Some(WOLF_PREYS),
            _ => None,
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
    pub fn skin(self, skin: &Skin) -> StyledChar {
        let mut s = self.kind.skin(skin).clone();
        if let Some(dir) = self.state.aim {
            s.set_char(match dir {
                Dir::Up => skin.aiming_up,
                Dir::Right => skin.aiming_right,
                Dir::Down => skin.aiming_down,
                Dir::Left => skin.aiming_left,
                _ => '?', // did I implement diagonal fire ?
            });
        }
        if self.state.drunk {
            s.set_fg(skin.drunk_color);
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
