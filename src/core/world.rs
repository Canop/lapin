use {
    crate::{
        pos::*,
    },
    super::*,
};

pub const FIRING_RANGE: Int = 6;

pub struct WorldPlayer<'t> {
    board: &'t Board,
    seed: usize,
}

impl<'t> WorldPlayer<'t> {
    pub fn new(board: &'t Board, seed: usize) -> Self {
        Self {
            board,
            seed,
        }
    }

    fn move_to_goal(
        &self,
        actor_id: ActorId,
        actor: Actor,
        goal: path::Goal,
    ) -> Option<ActorMove> {
        let mut path_finder = path::PathFinder::new(
            actor,
            &self.board,
            self.seed,
        );
        let hint = if actor.kind.runs_after(ActorKind::Lapin) {
            // waiting for bool.then_some to be not nightly (or better bool.map)
            Some(self.board.lapin_pos())
        } else {
            None
        };
        path_finder.find(goal, hint)
            .map(|path| path[0])
            .and_then(|pos| actor.pos.dir_to(pos))
            .map(|dir| ActorMove::new(actor_id, Action::Moves(dir)))
    }

    fn find_grazer_move(&self, actor_id: ActorId, actor: Actor) -> Option<ActorMove> {
        if self.board.get(actor.pos) == Terrain::Grass {
            None
        } else {
            self.move_to_goal(
                actor_id,
                actor,
                path::Goal::Terrain(Terrain::Grass),
            )
        }
    }

    fn find_lapin_eater_move(&self, actor_id: ActorId, actor: Actor) -> Option<ActorMove> {
        if let Some(dir) = actor.pos.dir_to(self.board.lapin_pos()) {
            // we can make a direct kill (may be a diagonal move)
            return Some(ActorMove::new(
                actor_id,
                Action::Eats(dir, 0),
            ));
        }
        self.move_to_goal(
            actor_id,
            actor,
            path::Goal::Pos(self.board.lapin_pos())
        )
    }

    // for actors who hunt several types of actors (not the fox)
    fn find_eater_move(&self, actor_id: ActorId, actor: Actor) -> Option<ActorMove> {
        for other_id in 0..self.board.actors.len() {
            if other_id == actor_id {
                continue;
            }
            let other = self.board.actors.by_id(other_id);
            if !actor.hits(other) {
                continue;
            }
            if let Some(dir) = actor.pos.dir_to(other.pos) {
                // we can make a direct kill (may be a diagonal move)
                if actor.can_enter(self.board.get(other.pos)) {
                    return Some(ActorMove::new(
                        actor_id,
                        Action::Eats(dir, other_id),
                    ));
                }
            }
        }
        self.move_to_goal(
            actor_id,
            actor,
            path::Goal::ActorKinds(actor.preys().unwrap()),
        )
    }

    fn nearest_fire_target(&self, actor_id: ActorId, actor: Actor) -> Option<(Actor, Int)> {
        let mut nearest_target: Option<(Actor, Int)> = None; // actor, distance
        for other_id in 0..self.board.actors.len() {
            if other_id == actor_id {
                continue;
            }
            let other = self.board.actors.by_id(other_id);
            if !actor.fires_on(other) {
                continue;
            }
            let dist = Pos::manhattan_distance(actor.pos, other.pos);
            if match nearest_target {
                Some((_, best_dist)) => best_dist > dist,
                _ => true
            } {
                nearest_target = Some((other, dist));
            }
        }
        nearest_target
    }

    fn can_enter(&self, actor: Actor, pos: Pos) -> bool {
        actor.can_enter(self.board.get(pos))
            && !self.board.actors.has_pos(pos)
    }


    fn find_firer_move(&self, actor_id: ActorId, actor: Actor) -> Option<ActorMove> {
        // we first check whether we have a target in the firing line
        if let Some(dir) = actor.state.aim {
            let mut pos = actor.pos;
            for _ in 0..FIRING_RANGE {
                pos = pos.in_dir(dir);
                if let Some((target_id, target)) = self.board.actors.id_actor_by_pos(pos) {
                    if actor.fires_on(target) {
                        // fire!
                        return Some(ActorMove::new(
                            actor_id,
                            Action::Fires(dir, target_id),
                        ));
                    }
                }
                if self.board.get(pos) == Terrain::Stone {
                    break;
                }
            }
        }
        // if there's a possible target in range, we try to lock aim on it
        let nearest_target = self.nearest_fire_target(actor_id, actor);
        if let Some((other, dist)) = nearest_target {
            if dist <= FIRING_RANGE {
                let quadrant_dir = actor.pos.quadrant_to(other.pos);
                // at this point we know the target isn't in the firing line
                // (or we would have fired)
                return match actor.state.aim {
                    // keep aiming
                    Some(dir) if dir==quadrant_dir => None,
                    // starts aiming
                    None => Some(ActorMove::new(actor_id, Action::Aims(quadrant_dir))),
                    // target lost
                    _ => Some(ActorMove::new(actor_id, Action::StopsAiming)),
                };
            }
        }
        // at this point there's nothing on which to aim or fire, so
        // we should walk
        if actor.is_aiming() {
            return Some(ActorMove::new(actor_id, Action::StopsAiming));
        }
        if actor.state.drunk {
            nearest_target
                .and_then(|(target, _)|
                    actor.pos
                        .quadrants_to(target.pos)
                        .iter()
                        .find(|&dir| self.can_enter(actor, actor.pos.in_dir(*dir)))
                        .map(|&dir| ActorMove::new(actor_id, Action::Moves(dir)))
                )
        } else {
            self.move_to_goal(
                actor_id,
                actor,
                path::Goal::ActorKinds(actor.preys().unwrap()),
            )
        }
    }

    fn find_actor_move(&self, actor_id: ActorId) -> Option<ActorMove> {
        use ActorKind::*;
        let actor = self.board.actors.by_id(actor_id);
        match actor.kind {
            Fox => self.find_lapin_eater_move(actor_id, actor),
            Knight | Wolf => self.find_eater_move(actor_id, actor),
            Hunter => self.find_firer_move(actor_id, actor),
            Sheep => self.find_grazer_move(actor_id, actor),
            _ => None, // No AI
        }
    }

    pub fn play(self) -> WorldMove {
        let mut actor_moves = Vec::new();
        // actor moves are computed sequentially so that the freed space
        // and the newly occupied place can be better taken into account.
        // Parallelizing would prevent optimal paths especially in case
        // of herds
        for id in 1..self.board.actors.len() {
            // let actor_move = time!(
            //     Debug,
            //     "move",
            //     format!("{:?}[{}]", actor.kind, id),
            //     self.find_actor_move(id),
            // );
            let actor_move = self.find_actor_move(id);
            if let Some(actor_move) = actor_move {
                actor_moves.push(actor_move);
            }
        }
        WorldMove { actor_moves }
    }
}
