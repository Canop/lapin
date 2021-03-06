use {
    crate::{
        pos::*,
        display::Skin,
    },
    serde::{Serialize, Deserialize},
    std::{
        fmt,
        hash::Hash,
    },
    super::*,
    termimad::{
        StyledChar,
    },
};

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, Hash)]
pub struct ActorState {

    #[serde(default)]
    pub dead: bool,

    pub aim: Option<Dir>,

    #[serde(default)]
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
    Dragon,
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
            Dragon => write!(f, "dragon"),
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
    ActorKind::Dragon,
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
pub static DRAGON_PREYS: &[ActorKind] = &[
    ActorKind::Lapin,
    ActorKind::Hunter,
    ActorKind::Wolf,
    ActorKind::Fox,
    ActorKind::Sheep,
];
impl ActorKind {
    pub fn drinks_wine(self) -> bool {
        match self {
            ActorKind::Hunter => true,
            _ => false,
        }
    }
    pub fn eats(self, other: Self) -> bool {
        use ActorKind::*;
        match (self, other) {
            (Fox, Lapin) => true,
            (Knight, Fox) => true,
            (Knight, Hunter) => true,
            (Knight, Wolf) => true,
            (Wolf, Hunter) => true,
            (Wolf, Sheep) => true,
            (Wolf, Lapin) => true,
            (Dragon, _) => true,
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
            (Dragon, _) => true,
            _ => false,
        }
    }
    pub fn is_immune_to_fire(self, firer: ActorKind) -> bool {
        match (self, firer) {
            (_, Self::Dragon) => false,
            (Self::Knight, _) => true,
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
            Dragon => &skin.dragon,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Hash)]
pub struct Actor {
    pub kind: ActorKind,

    pub pos: Pos,

    #[serde(skip)]
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
            (Dragon, Water) => true, // dragon flies over water
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
            Dragon => Some(DRAGON_PREYS),
            _ => None,
        }
    }
    pub fn eats(self, other: Actor) -> bool {
        self.kind.eats(other.kind)
    }
    pub fn fires_on(self, other: Actor) -> bool {
        use ActorKind::*;
        match self.kind {
            Dragon => true,
            Hunter if self.state.drunk => true,
            Hunter => match other.kind {
                Fox | Knight | Lapin | Wolf => true,
                _ => false,
            }
            _ => false,
        }
        // soon : https://github.com/rust-lang/rust/issues/54883
        // match (self.kind, other.kind) {
        //     (Hunter, _) if self.state.drunk => true,
        //     (Hunter, Fox | Knight | Lapin | Wolf) => true,
        //     _ => false,
        // }
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
