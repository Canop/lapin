
use {
    crate::{
        pos::*,
    },
    super::*,
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

#[derive(Debug, Clone, Copy)]
pub enum Goal {
    Pos(Pos),
    Terrain(Terrain),
    ActorKinds(&'static[ActorKind]),
}

pub struct PathFinder<'b> {
    actor: Actor,
    board: &'b Board,
    seed: usize,
}

impl<'b> PathFinder<'b> {
    pub fn new(
        actor: Actor,
        board: &'b Board,
        seed: usize,
    ) -> Self {
        Self {
            actor,
            board,
            seed,
        }
    }

    /// tells whether the terrain can be an intermediate step on the path.
    // This function will usually return false for the goal. It's
    // thus necessary to check the goal before calling this one.
    fn can_enter(&self, pos: Pos) -> bool {
        self.actor.can_enter(self.board.get(pos)) && !self.board.actors.has_pos(pos)
    }

    /// tells whether the pos is a/the goal
    fn reached(&self, pos: Pos, goal: Goal) -> bool {
        match goal {
            Goal::Pos(goal_pos) => goal_pos == pos,
            Goal::Terrain(terrain) => self.board.get(pos) == terrain && !self.board.actors.has_pos(pos),
            Goal::ActorKinds(kinds) => self.board.actors.by_pos(pos)
                .map_or(false, |actor| kinds.contains(&actor.kind) && self.actor.can_enter(self.board.get(pos))),
        }
    }

    pub fn find(
        &mut self,
        goal: Goal,
        hint: Option<Pos>,
    ) -> Option<Vec<Pos>> {
        match hint {
            // when we target a precise position or have a global direction,
            // there's nothing better than A*. This is especially true
            // in open spaces when there's no limits to moves
            Some(pos) => self.find_astar(goal, pos),

            // general dijkstra
            None => self.find_dijkstra(goal),
        }
    }

    /// find the shortest path to any terrain verifying the goal.
    /// Doesn't use any heuristic so is slower than A* but works
    /// for many goals.
    /// This function is based on Dijkstra's algorithm.
    fn find_dijkstra(
        &mut self,
        goal: Goal,
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
            closed_set.insert(current);
            for i in 0..4 {
                // not always trying the same path when several are identical
                // in lenght avoids some locking situation and some abuses
                let dir = DIRS[(i + self.seed)%4];
                let neighbour = current.in_dir(dir);
                if self.reached(neighbour, goal) {
                    // reconstruct the path from current to start
                    let mut path = vec![neighbour];
                    while current != start {
                        path.push(current);
                        current = came_from.get(current);
                    }
                    path.reverse();
                    return Some(path);
                }
                if !self.can_enter(neighbour) || closed_set.has_key(neighbour) {
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

    /// Find a short path between start and goal using A*.
    ///
    /// The returned path contains the goal but not the start.
    ///
    /// If the goal is different from the hinted pos, the path
    /// found may not be the absolute best one (i.e. a different
    /// goal in opposite direction of the hint can be nearer).
    ///
    /// The heuristic function h used here is the Euclidian distance
    /// to the hint (which may be the goal).
    fn find_astar(
        &mut self,
        goal: Goal,
        hint: Pos,
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
        open_set.push(ValuedPos::from(start, 0));

        while let Some(mut current) = open_set.pop().map(|vp| vp.pos) {
            closed_set.insert(current);
            for i in 0..4 {
                // not always trying the same path when several are identical
                // in lenght avoids some locking situation and some abuses
                let dir = DIRS[(i + self.seed)%4];
                let neighbour = current.in_dir(dir);
                if self.reached(neighbour, goal) {
                    // reconstruct the path from current to start
                    let mut path = vec![neighbour];
                    while current != start {
                        path.push(current);
                        current = came_from.get(current);
                    }
                    path.reverse();
                    return Some(path);
                }
                if !self.can_enter(neighbour) || closed_set.has_key(neighbour) {
                    continue;
                }
                let tentative_g_score = g_score.get(current) + 1;
                let previous_g_score = g_score.get(neighbour);
                if tentative_g_score < previous_g_score {
                    came_from.set(neighbour, current);
                    g_score.set(neighbour, tentative_g_score);
                    let new_f_score = tentative_g_score + 2 * Pos::euclidian_distance(neighbour, hint);
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


