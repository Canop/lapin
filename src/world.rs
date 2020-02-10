use {
    crate::{
        actor::*,
        board::Board,
        path::PathFinder,
        pos::*,
    },
    std::{
        collections::{
            HashMap,
        },
    },
};

/// what the world plays in a non-player turn.
/// Arrays here must be consistent with the board.
#[derive(Debug)]
pub struct WorldMove {
    pub actor_moves: Vec<ActorMove>,
}

pub struct WorldPlayer<'t> {
    board: &'t Board,
    actors_map: HashMap<Pos, Actor>,
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
    fn find_actor_move(&self, actor_id: usize) -> Option<ActorMove> {
        let actor = self.board.actors[actor_id];
        let mut goals: Vec<Pos> = Vec::new();
        for (other_id, other) in self.board.actors.iter().enumerate() {
            if self.killed[other_id] || other_id == actor_id || !actor.eats(*other) {
                continue;
            }
            if let Some(dir) = actor.pos.dir_to(other.pos) {
                // we can make a direct kill (may be a diagonal move)
                return Some(ActorMove {
                    actor_id,
                    target_id: Some(other_id),
                    dir,
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
                    dir,
                }
            )
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
                self.actors_map.remove(&actor.pos);
                self.actors_map.insert(actor.pos.in_dir(actor_move.dir), actor);
                actor_moves.push(actor_move);
            }
        }
        WorldMove { actor_moves }
    }
}
