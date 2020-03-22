
use {
    anyhow::Result,
    crate::{
        pos::{
            Dir,
            Pos,
            PosArea,
            PosMap,
        },
    },
    std::{
        ops::Range,
    },
    super::{
        Actor,
        ActorKind,
        ActorState,
    },
};

/// a non nullable id of an actor, for use in the
/// actor_map. An ActorId is only valid until the
/// next "remove" operation (it's the index in the
/// underlying array)
pub type ActorId = usize;


/// an option of an id, a little more compact
#[derive(Debug, Clone, Copy)]
pub struct ActorRef {
    id: usize,
}

impl ActorRef {
    pub fn none() -> Self {
        Self { id: std::usize::MAX }
    }
    pub fn from_id(id: ActorId) -> Self {
        Self { id }
    }
    pub fn lapin() -> Self {
        Self::from_id(0)
    }
    pub fn is_some(self) -> bool {
        self.id != std::usize::MAX
    }
    pub fn is_none(self) -> bool {
        self.id == std::usize::MAX
    }
    pub fn id(self) -> Option<ActorId> {
        if self.is_some() {
            Some(self.id)
        } else {
            None
        }
    }
}

/// opaque structure keeping all actors.
/// It ensures consistency (of position) by not
/// allowing direct access to the actors
#[derive(Clone)]
pub struct ActorMap {
    actors: Vec<Actor>,
    ref_pos_map: PosMap<ActorRef>,
}

impl ActorMap {
    pub fn new(area: PosArea, actors: Vec<Actor>) -> Self {
        let mut ref_pos_map = PosMap::new(area, ActorRef::none());
        for (i, a) in actors.iter().enumerate() {
            ref_pos_map.set(a.pos, ActorRef::from_id(i));
        }
        Self {
            actors,
            ref_pos_map,
        }
    }
    pub fn add(&mut self, actor: Actor) -> Result<ActorRef> {
        if let Some(actor) = self.by_pos(actor.pos) {
            if !actor.state.dead {
                Err(anyhow!("pos already occupied by {}", actor.kind))?;
            }
        }
        let ar = ActorRef::from_id(self.actors.len());
        self.actors.push(actor);
        self.ref_pos_map.set(actor.pos, ar);
        Ok(ar)
    }
    pub fn ref_by_pos(&self, pos: Pos) -> ActorRef {
        self.ref_pos_map.get(pos)
    }
    pub fn id_actor_by_pos(&self, pos: Pos) -> Option<(ActorId, Actor)> {
        self.ref_pos_map.get(pos).id().map(|id|
            (id, self.actors[id])
        )
    }
    pub fn by_pos(&self, pos: Pos) -> Option<Actor> {
        self.by_ref(self.ref_pos_map.get(pos))
    }
    pub fn by_ref(&self, ar: ActorRef) -> Option<Actor> {
        ar.id().map(|idx| self.actors[idx])
    }
    pub fn by_id(&self, id: ActorId) -> Actor {
        self.actors[id]
    }
    pub fn has_pos(&self, pos: Pos) -> bool {
        self.by_pos(pos).map_or(false, |a| !a.state.dead)
    }
    pub fn state_by_id_mut(&mut self, id: ActorId) -> &mut ActorState {
        unsafe {
            &mut self.actors.get_unchecked_mut(id).state
        }
    }
    pub fn lapin(&self) -> Actor {
        self.actors[0]
    }
    pub fn move_lapin_to(&mut self, dest: Pos) {
        self.ref_pos_map.unset(self.actors[0].pos);
        self.actors[0].pos = dest;
        self.ref_pos_map.set(dest, ActorRef::lapin());
    }
    /// move the actor in the given direction
    pub fn move_by_id_in_dir(&mut self, id: ActorId, dir: Dir) -> Result<()> {
        let new_pos = self.actors[id].pos.in_dir(dir);
        self.move_by_id_to_pos(id, new_pos)
    }
    /// move the actor to the new position
    /// If there's already one actor in new_pos, one of them must
    /// be dead (or the move is prevented and an error is thrown)
    pub fn move_by_id_to_pos(&mut self, id: ActorId, new_pos: Pos) -> Result<()> {
        if let Some(actor) = self.by_pos(new_pos) {
            if !self.actors[0].state.dead && !actor.state.dead {
                return Err(anyhow!("pos already occupied by {}", actor.kind))?;
            }
        }
        self.ref_pos_map.unset(self.actors[id].pos); // we should test we're unsetting the right one
        self.actors[id].pos = new_pos;
        if !self.actors[id].state.dead {
            self.ref_pos_map.set(new_pos, ActorRef::from_id(id));
        }
        Ok(())
    }
    pub fn iter(&self) -> std::slice::Iter<Actor> {
        self.actors.iter()
    }
    pub fn len(&self) -> usize {
        self.actors.len()
    }
    pub fn ai_actor_ids(&self) -> Range<ActorId> {
        1..self.actors.len()
    }
    fn rebuild(&mut self) {
        self.ref_pos_map.clear();
        for (i, a) in self.actors.iter().enumerate() {
            self.ref_pos_map.set(a.pos, ActorRef::from_id(i));
        }
    }
    pub fn remove_by_id(&mut self, id: ActorId) {
        if id == 0 {
            warn!("attempt at removing the lapin");
            return;
        }
        self.ref_pos_map.unset(self.actors[id].pos);
        self.actors.swap_remove(id);
        if id < self.len() {
            // an actor changed id because of the swap_remove, we update the ref
            self.ref_pos_map.set(self.actors[id].pos, ActorRef::from_id(id));
        }
    }
    /// Clean the map from actor in pos, if any.
    /// Note that existing ActorId aren't valid anymore after this change.
    pub fn remove_by_pos(&mut self, pos: Pos) {
        if let Some(id) = self.ref_by_pos(pos).id() {
            self.remove_by_id(id);
        }
    }
    /// Clean the map from actors whose state.dead is true.
    /// Note that existing ActorId aren't valid anymore after this change.
    /// don't remove the lapin because we need it for positionning the screen
    pub fn remove_dead(&mut self) {
        let mut i = self.actors.len() - 1;
        let mut nb_removed = 0;
        while i > 0 { // don't remove the lapin
            if self.actors[i].state.dead {
                //self.remove_by_id(i);
                self.actors.remove(i);
                nb_removed += 1;
            }
            i -= 1;
        }
        if nb_removed > 0 {
            debug!("removed {} dead actors", nb_removed);
            self.rebuild();
        }
    }
    pub fn to_vec(self) -> Vec<Actor> {
        self.actors
    }
    pub fn vec(&self) -> Vec<Actor> {
        self.actors.clone()
    }
}

impl From<PosArea> for ActorMap {
    fn from(area: PosArea) -> Self {
        let lapin = Actor::new(ActorKind::Lapin, 0, 0);
        let actors = vec![lapin];
        let mut ref_pos_map = PosMap::new(area, ActorRef::none());
        ref_pos_map.set(lapin.pos, ActorRef::from_id(0));
        Self {
            actors,
            ref_pos_map,
        }
    }
}

