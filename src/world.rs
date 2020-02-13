use {
    crate::{
        actor::*,
        board::Board,
        path::PathFinder,
        pos::*,
    },
    fnv::{
        FnvHashMap,
    },
    std::{
        collections::{
            HashMap,
        },
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
impl ActorMove {
    //pub fn target_id(self) -> Option<usize> {
    //    match self.action {
    //        Action::Eats(_, k) => Some(k),
    //        Action::Eats(_, k) => Some(k),
    //        _ => None,
    //    }
    //}
}

/// what the world plays in a non-player turn.
/// Arrays here must be consistent with the board.
#[derive(Debug)]
pub struct WorldMove {
    pub actor_moves: Vec<ActorMove>,
}

pub struct WorldPlayer<'t> {
    board: &'t Board,
    actors_map: FnvHashMap<Pos, Actor>,
    killed: Vec<bool>,
}
impl<'t> WorldPlayer<'t> {
    pub fn new(board: &'t Board) -> Self {
        let actors_map = board.actors_map();
        let killed = vec![false; board.actors.len()];
        Self {
            board,
            actors_map,
            killed,
        }
    }
    fn actor_pos(&self, actor_id: usize) -> Pos {
        self.board.actors[actor_id].pos
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
        let path_finder = PathFinder::new(actor, &self.board, &self.actors_map);
        path_finder.find_to_vec(&goals)
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
                debug!("target in range");
                let quadrant_dir = actor.pos.quadrant_to(other.pos);
                return match actor.state {
                    ActorState::Aiming(dir) => {
                        // is the target in the firing line ?
                        if actor.pos.is_in_dir(other.pos, dir) {
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
                    }
                    _ => {
                        // starts aiming
                        Some(ActorMove {
                            actor_id,
                            target_id: None,
                            action: Action::Aims(quadrant_dir),
                        })
                    }
                };
            }
        }
        if actor.is_aiming() {
            Some(ActorMove {
                actor_id,
                target_id: None,
                action: Action::StopsAiming,
            })
        } else {
            let path_finder = PathFinder::new(actor, &self.board, &self.actors_map);
            path_finder.find_to_vec(&goals)
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
    }

    // right now, AI played actors are either eaters (contact) or firers (range)
    // so we optimize computations by doing one or the other depending on the type
    fn find_actor_move(&self, actor_id: usize) -> Option<ActorMove> {
        let actor = self.board.actors[actor_id];
        match actor.kind {
            ActorKind::Fox | ActorKind::Knight | ActorKind::Wolf => {
                self.find_eater_move(actor_id, actor)
            }
            ActorKind::Hunter => self.find_firer_move(actor_id, actor),
            _ => None, // No AI
        }
    }

    pub fn play(mut self) -> WorldMove {
        let mut actor_moves = Vec::new();
        for id in 1..self.board.actors.len() {
            let actor = self.board.actors[id];
            if self.killed[id] {
                continue;
            }
            if let Some(actor_move) = self.find_actor_move(id) {
                if let Some(other_id) = actor_move.target_id {
                    self.killed[other_id] = true;
                    self.actors_map.remove(&self.actor_pos(other_id));
                }
                match actor_move.action {
                    Action::Eats(dir) | Action::Moves(dir) => {
                        self.actors_map.remove(&actor.pos);
                        self.actors_map.insert(actor.pos.in_dir(dir), actor);
                    }
                    _ => {}
                }
                actor_moves.push(actor_move);
            }
        }
        WorldMove { actor_moves }
    }
}
