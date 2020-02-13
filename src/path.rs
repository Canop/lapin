
use {
    crate::{
        actor::*,
        board::Board,
        pos::*,
    },
    fnv::{
        FnvHashMap,
        FnvHashSet,
    },
    std::{
        collections::{
            HashMap,
            HashSet,
        },
    },
};

const MAX_OPEN_SIZE: usize = 200;
const INFINITY: Int = 32_767;

pub struct PathFinder<'b> {
    actor: Actor,
    board: &'b Board,
    actors_map: &'b FnvHashMap<Pos, Actor>,
}

impl<'b> PathFinder<'b> {
    pub fn new(
        actor: Actor,
        board: &'b Board,
        actors_map: &'b FnvHashMap<Pos, Actor>,
    ) -> Self {
        Self {
            actor,
            board,
            actors_map,
        }
    }
    pub fn can_enter(&self, pos: Pos) -> bool {
        self.board.is_enterable(pos) && !self.actors_map.contains_key(&pos)
    }
    pub fn dirs(&self) -> impl Iterator<Item = Dir> {
        [Dir::Up, Dir::Right, Dir::Down, Dir::Left].iter().copied()
    }

    pub fn find_to_vec(
        &self,
        goals: &[Pos],
    ) -> Option<Vec<Pos>> {
        let mut shortest: Option<Vec<Pos>> = None;
        for goal in goals {
            if let Some(path) = self.find(*goal) {
                shortest = Some(if let Some(sh) = shortest {
                    if sh.len() < path.len() {
                        sh
                    } else {
                        path
                    }
                } else {
                    path
                });
            }
        }
        shortest
    }

    /// Find the shortest path between start and goal using A*.
    ///
    /// The returned path contains the goal but not the start.
    ///
    /// This is a quite direct implementation of the pseudocode at
    /// https://en.wikipedia.org/wiki/A*_search_algorithm
    /// and I try to keep the same names.
    /// The heuristic function h used here is the Manhattan distance
    /// to the goal.
    pub fn find(
        &self,
        goal: Pos,
    ) -> Option<Vec<Pos>> {

        let start = self.actor.pos;

        // nodes already evaluated, we know they're not interesting
        let mut closed_set: FnvHashSet<Pos> = FnvHashSet::default();

        // node immediately preceding on the cheapest known path from start
        let mut came_from: FnvHashMap<Pos, Pos> = FnvHashMap::default();

        // g_score is the cost of the cheapest path from start to a pos
        let mut g_score: FnvHashMap<Pos, Int> = FnvHashMap::default(); // infinite when missing
        g_score.insert(start, 0);

        // f_score = g_score + h_score
        let mut f_score: FnvHashMap<Pos, Int> = FnvHashMap::default(); // infinite when missing
        f_score.insert(start, Pos::manhattan_distance(start, goal));

        // the nodes from which we may expand. All nodes in this
        // set have a f_score and a g_score by construct
        let mut open_set: FnvHashSet<Pos> = FnvHashSet::default();
        open_set.insert(start);

        while let Some(current) = open_set.iter().min_by_key(|p| f_score.get(p).unwrap()) {
            let mut current = *current;
            if current == goal {
                // reconstruct the path from current to start
                let mut path = Vec::new();
                while current != start {
                    path.push(current);
                    current = *came_from.get(&current).unwrap();
                }
                path.reverse();
                return Some(path);
            }
            open_set.remove(&current);
            closed_set.insert(current);
            for dir in self.dirs() {
                let neighbour = current.in_dir(dir);
                if
                    neighbour != goal
                    && ( !self.can_enter(neighbour) || closed_set.contains(&neighbour) )
                {
                    continue;
                }
                let tentative_g_score = g_score.get(&current).unwrap() + 1;
                let previous_g_score = *g_score.get(&neighbour).unwrap_or(&INFINITY);
                if tentative_g_score < previous_g_score {
                    came_from.insert(neighbour, current);
                    g_score.insert(neighbour, tentative_g_score);
                    let new_f_score = tentative_g_score + Pos::manhattan_distance(neighbour, goal);
                    f_score.insert(neighbour, new_f_score);
                    open_set.insert(neighbour);
                }
            }
            if open_set.len() > MAX_OPEN_SIZE {
                debug!("open set too big");
                break;
            }
        }

        // open_set is empty, there's no path
        None
    }
}


