use {
    crate::{
        actor::*,
        board::Board,
        consts::*,
        path::PathFinder,
        pos::*,
    },
};

pub const FIRING_RANGE: Int = 6;

#[derive(Debug, Clone, Copy)]
pub enum Action {
    Moves(Dir),
    Aims(Dir),
    StopsAiming,
    Eats(Dir),
    Fires(Dir),
}

#[derive(Debug, Clone, Copy)]
pub struct ActorMove {
    pub actor_id: usize,
    pub target_id: Option<usize>, // the actor killed by the move
    pub action: Action,
}

/// what the world plays in a non-player turn.
/// Arrays here must be consistent with the board.
#[derive(Debug)]
pub struct WorldMove {
    pub actor_moves: Vec<ActorMove>,
}

pub struct WorldPlayer<'t> {
    board: &'t Board,
    actor_pos_set: PosSet,
    killed: Vec<bool>,
    seed: usize,
}

impl<'t> WorldPlayer<'t> {
    pub fn new(board: &'t Board, seed: usize) -> Self {
        let actor_pos_set = board.actor_pos_set();
        let killed = vec![false; board.actors.len()];
        Self {
            board,
            actor_pos_set,
            killed,
            seed,
        }
    }
    fn actor_pos(&self, actor_id: usize) -> Pos {
        self.board.actors[actor_id].pos
    }

    pub fn is_firing_dir(
        &self,
        mut pos: Pos,
        dir: Dir,
        target: Pos,
    ) -> bool {
        for _ in 0..FIRING_RANGE {
            pos = pos.in_dir(dir);
            if pos == target {
                return true;
            }
            if self.board.get(pos) == WALL {
                return false;
            }
        }
        false
    }

    fn move_to_vec(
        &self,
        actor_id: usize,
        actor: Actor,
        mut goals: Vec<Pos>,
    ) -> Option<ActorMove> {
        let mut path_finder = PathFinder::new(
            actor,
            &self.board,
            &self.actor_pos_set,
            self.seed,
            actor.path_finding_strategy(),
        );
        path_finder.find_to_vec(&mut goals)
            .map(|path| path[0])
            .and_then(|pos| actor.pos.dir_to(pos))
            .map(|dir|
                ActorMove {
                    actor_id,
                    target_id: None,
                    action: Action::Moves(dir),
                }
            )
    }

    fn find_grazer_move(&self, actor_id: usize, actor: Actor) -> Option<ActorMove> {
        if self.board.get(actor.pos) == GRASS {
            None
        } else {
            let mut goals: Vec<Pos> = Vec::new();
            for &p in &self.board.grass_cells {
                if !self.actor_pos_set.has_key(p) {
                    goals.push(p);
                }
            }
            self.move_to_vec(actor_id, actor, goals)
        }
    }

    fn find_eater_move(&self, actor_id: usize, actor: Actor) -> Option<ActorMove> {
        let mut goals: Vec<Pos> = Vec::new();
        for (other_id, other) in self.board.actors.iter().enumerate() {
            if other_id == actor_id || self.killed[other_id] || !actor.hits(*other) {
                continue;
            }
            if let Some(dir) = actor.pos.dir_to(other.pos) {
                // we can make a direct kill (may be a diagonal move)
                return Some(ActorMove {
                    actor_id,
                    target_id: Some(other_id),
                    action: Action::Eats(dir),
                });
            }
            goals.push(other.pos);
        }
        self.move_to_vec(actor_id, actor, goals)
    }

    fn find_firer_move(&self, actor_id: usize, actor: Actor) -> Option<ActorMove> {
        let mut goals: Vec<Pos> = Vec::new();
        for (other_id, other) in self.board.actors.iter().enumerate() {
            if other_id == actor_id || self.killed[other_id] {
                continue;
            }
            if actor.runs_after(*other) {
                goals.push(other.pos);
            } else if !actor.fires_on(*other) {
                continue;
            }
            let dist = Pos::manhattan_distance(actor.pos, other.pos);
            if dist <= FIRING_RANGE {
                let quadrant_dir = actor.pos.quadrant_to(other.pos);
                return if let Some(dir) = actor.state.aim {
                    // is the target in the firing line ?
                    if self.is_firing_dir(actor.pos, dir, other.pos) {
                        // fire!
                        Some(ActorMove {
                            actor_id,
                            target_id: Some(other_id),
                            action: Action::Fires(quadrant_dir),
                        })
                    } else if dir != quadrant_dir {
                        // target lost
                        Some(ActorMove {
                            actor_id,
                            target_id: None,
                            action: Action::StopsAiming,
                        })
                    } else {
                        // go on aiming
                        None
                    }
                } else {
                    // starts aiming
                    Some(ActorMove {
                        actor_id,
                        target_id: None,
                        action: Action::Aims(quadrant_dir),
                    })
                }
            }
        }
        if actor.is_aiming() {
            Some(ActorMove {
                actor_id,
                target_id: None,
                action: Action::StopsAiming,
            })
        } else {
            self.move_to_vec(actor_id, actor, goals)
        }
    }

    // right now, AI played actors are either eaters (contact) or firers (range)
    // so we optimize computations by doing one or the other depending on the type
    fn find_actor_move(&self, actor_id: usize) -> Option<ActorMove> {
        use ActorKind::*;
        let actor = self.board.actors[actor_id];
        match actor.kind {
            Knight | Fox | Wolf => self.find_eater_move(actor_id, actor),
            Hunter => self.find_firer_move(actor_id, actor),
            Sheep => self.find_grazer_move(actor_id, actor),
            _ => None, // No AI
        }
    }

    pub fn play(mut self) -> WorldMove {
        let mut actor_moves = Vec::new();
        // actor moves are computed sequentially so that the freed space
        // and the newly occupied place can be better taken into account.
        // Parallelizing would prevent optimal paths especially in case
        // of herds
        for id in 1..self.board.actors.len() {
            let actor = self.board.actors[id];
            if self.killed[id] {
                continue;
            }
            if let Some(actor_move) = self.find_actor_move(id) {
                if let Some(other_id) = actor_move.target_id {
                    self.killed[other_id] = true;
                    self.actor_pos_set.remove(self.actor_pos(other_id));
                }
                match actor_move.action {
                    Action::Eats(dir) | Action::Moves(dir) => {
                        self.actor_pos_set.remove(actor.pos);
                        self.actor_pos_set.insert(actor.pos.in_dir(dir));
                    }
                    _ => {}
                }
                actor_moves.push(actor_move);
            }
        }
        WorldMove { actor_moves }
    }
}
