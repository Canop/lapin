
use {
    crate::{
        actor::*,
        board::Board,
        consts::Cell,
        pos::*,
    },
    std::{
        collections::{
            BinaryHeap,
            VecDeque,
        },
        cmp::{
            Ordering,
        },
    },
};

const MAX_OPEN_SIZE: usize = 200;
const INFINITY: Int = 32_767;

static DIRS: [Dir; 4] = [Dir::Up, Dir::Right, Dir::Down, Dir::Left];

#[derive(Debug, Clone, Copy)]
struct ValuedPos {
    pos: Pos,
    score: Int,
}
impl ValuedPos {
    pub fn from(pos: Pos, score: Int) -> Self {
        ValuedPos { pos, score }
    }
}
impl Eq for ValuedPos {}
impl PartialEq for ValuedPos {
    fn eq(&self, other: &ValuedPos) -> bool {
        self.score == other.score
    }
}
// we order in reverse from score
impl Ord for ValuedPos {
    fn cmp(&self, other: &ValuedPos) -> Ordering {
        other.score.cmp(&self.score)
    }
}
impl PartialOrd for ValuedPos {
    fn partial_cmp(&self, other: &ValuedPos) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub enum PathFindingStrategy {
    Quadrant, // just try to go in the general direction of the nearest
    BestToNearest,
    Best, // absolute best, slower
}

pub struct PathFinder<'b> {
    actor: Actor,
    board: &'b Board,
    actors_map: &'b PosSet,
    seed: usize,
    strategy: PathFindingStrategy,
}

impl<'b> PathFinder<'b> {
    pub fn new(
        actor: Actor,
        board: &'b Board,
        actors_map: &'b PosSet,
        seed: usize,
        strategy: PathFindingStrategy,
    ) -> Self {
        Self {
            actor,
            board,
            actors_map,
            seed,
            strategy,
        }
    }
    pub fn can_enter(&self, pos: Pos) -> bool {
        self.board.is_enterable(pos) && !self.actors_map.has_key(pos)
    }

    // When there are many goals, this implementation could be replaced
    // by a unique search with the heuristic based on the distance to
    // the nearest goal (which could be precomputed)
    pub fn find_to_vec(
        &mut self,
        goals: &mut[Pos],
    ) -> Option<Vec<Pos>> {
        let actor_pos = self.actor.pos;
        match self.strategy {
            PathFindingStrategy::Quadrant => {
                goals.sort_by(|&a, &b|
                    Pos::manhattan_distance(a, actor_pos).cmp(&Pos::manhattan_distance(b, actor_pos))
                );
                goals.get(0).and_then(|&goal| {
                    let pos = actor_pos.in_dir(actor_pos.quadrant_to(goal));
                    if self.can_enter(pos) {
                        Some(vec![pos; 1])
                    } else {
                        None
                    }
                })
            }
            PathFindingStrategy::BestToNearest => {
                goals.sort_by(|&a, &b|
                    Pos::manhattan_distance(a, self.actor.pos).cmp(&Pos::manhattan_distance(b, self.actor.pos))
                );
                for goal in goals {
                    if let Some(path) = self.find_to_pos(*goal) {
                        return Some(path);
                    }
                }
                None
            }
            PathFindingStrategy::Best => {
                let mut shortest: Option<Vec<Pos>> = None;
                for goal in goals {
                    if let Some(path) = self.find_to_pos(*goal) {
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
        }
    }

    /// find the shortest path to a cell of a given nature
    /// (used by sheeps to find grass).
    /// It's based on Dijkstra's algorithm.
    pub fn find_to_terrain(
        &mut self,
        goal: Cell,
    ) -> Option<Vec<Pos>> {
        let start = self.actor.pos;

        // nodes already evaluated, we know they're not interesting
        let mut closed_set = PosSet::from(self.board.area.clone());

        // node immediately preceding on the cheapest known path from start
        let mut came_from: PosMap<Pos> = PosMap::new(self.board.area.clone(), start);

        // g_score is the cost of the cheapest path from start to a pos
        let mut g_score: PosMap<Int> = PosMap::new(self.board.area.clone(), INFINITY);
        g_score.set(start, 0);

        // the nodes from which we may expand
        let mut open_set: VecDeque<Pos> = VecDeque::new();
        open_set.push_back(start);

        while let Some(mut current) = open_set.pop_front() {
            if self.board.get(current) == goal {
                // reconstruct the path from current to start
                let mut path = Vec::new();
                while current != start {
                    path.push(current);
                    current = came_from.get(current);
                }
                path.reverse();
                return Some(path);
            }
            closed_set.insert(current);
            for i in 0..4 {
                // not always trying the same path when several are identical
                // in lenght avoids some locking situation and some abuses
                let dir = DIRS[(i + self.seed)%4];
                let neighbour = current.in_dir(dir);
                if
                    self.board.get(neighbour) != goal // TODO check if faster if short return here
                    && ( !self.can_enter(neighbour) || closed_set.has_key(neighbour) )
                {
                    continue;
                }
                let tentative_g_score = g_score.get(current) + 1;
                let previous_g_score = g_score.get(neighbour);
                if tentative_g_score < previous_g_score {
                    came_from.set(neighbour, current);
                    g_score.set(neighbour, tentative_g_score);
                    open_set.push_back(neighbour);
                }
            }
            if open_set.len() > MAX_OPEN_SIZE {
                debug!("open set too big");
                break;
            }
            self.seed = (self.seed + 1) % 27;
        }

        // open_set is empty, there's no path
        None
    }

    /// Find the shortest path between start and goal using A*.
    ///
    /// The returned path contains the goal but not the start.
    ///
    /// The heuristic function h used here is the Manhattan distance
    /// to the goal.
    pub fn find_to_pos(
        &mut self,
        goal: Pos,
    ) -> Option<Vec<Pos>> {
        let start = self.actor.pos;

        // nodes already evaluated, we know they're not interesting
        let mut closed_set = PosSet::from(self.board.area.clone());

        // node immediately preceding on the cheapest known path from start
        let mut came_from: PosMap<Pos> = PosMap::new(self.board.area.clone(), start);

        // g_score is the cost of the cheapest path from start to a pos
        let mut g_score: PosMap<Int> = PosMap::new(self.board.area.clone(), INFINITY);
        g_score.set(start, 0);

        // the nodes from which we may expand
        let mut open_set: BinaryHeap<ValuedPos> = BinaryHeap::new();
        open_set.push(ValuedPos::from(start, Pos::manhattan_distance(start, goal)));

        while let Some(mut current) = open_set.pop().map(|vp| vp.pos) {
            if current == goal {
                // reconstruct the path from current to start
                let mut path = Vec::new();
                while current != start {
                    path.push(current);
                    current = came_from.get(current);
                }
                path.reverse();
                return Some(path);
            }
            closed_set.insert(current);
            for i in 0..4 {
                // not always trying the same path when several are identical
                // in lenght avoids some locking situation and some abuses
                let dir = DIRS[(i + self.seed)%4];
                let neighbour = current.in_dir(dir);
                if
                    neighbour != goal
                    && ( !self.can_enter(neighbour) || closed_set.has_key(neighbour) )
                {
                    continue;
                }
                let tentative_g_score = g_score.get(current) + 1;
                let previous_g_score = g_score.get(neighbour);
                if tentative_g_score < previous_g_score {
                    came_from.set(neighbour, current);
                    g_score.set(neighbour, tentative_g_score);
                    let new_f_score = tentative_g_score + Pos::manhattan_distance(neighbour, goal);
                    open_set.push(ValuedPos::from(neighbour, new_f_score));
                }
            }
            if open_set.len() > MAX_OPEN_SIZE {
                debug!("open set too big");
                break;
            }
            self.seed = (self.seed + 1) % 27;
        }

        // open_set is empty, there's no path
        None
    }
}


