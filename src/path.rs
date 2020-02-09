
use {
    crate::{
        board::Board,
        pos::*,
    },
    std::{
        collections::{
            HashMap,
            HashSet,
        },
    },
};

const MAX_OPEN_SIZE: usize = 1000;
const INFINITY: Int = 32_767;

pub struct PathFinder<'b> {
    board: &'b Board,
    taken_set: HashSet<Pos>,
}

impl<'b> PathFinder<'b> {
    pub fn new(board: &'b Board) -> Self {
        let taken_set = HashSet::new();
        Self {
            board,
            taken_set,
        }
    }
    pub fn reserve(&mut self, pos: Pos) {
        self.taken_set.insert(pos);
    }
    pub fn can_enter(&self, pos: Pos) -> bool {
        self.board.is_enterable(pos) && !self.taken_set.contains(&pos)
    }
    pub fn dirs(&self) -> impl Iterator<Item = Dir> {
        [Dir::Up, Dir::Right, Dir::Down, Dir::Left].iter().copied()
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
        start: Pos,
        goal: Pos,
    ) -> Option<Vec<Pos>> {

        // nodes already evaluated, we know they're not interesting
        let mut closed_set: HashSet<Pos> = HashSet::new();

        // node immediately preceding on the cheapest known path from start
        let mut came_from: HashMap<Pos, Pos> = HashMap::new();

        // g_score is the cost of the cheapest path from start to a pos
        let mut g_score: HashMap<Pos, Int> = HashMap::new(); // infinite when missing
        g_score.insert(start, 0);

        // f_score = g_score + h_score
        let mut f_score: HashMap<Pos, Int> = HashMap::new(); // infinite when missing
        f_score.insert(start, Pos::manhattan_distance(start, goal));

        // the nodes from which we may expand. All nodes in this
        // set have a f_score and a g_score by construct
        let mut open_set: HashSet<Pos> = HashSet::new();
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
                if !self.can_enter(neighbour) || closed_set.contains(&neighbour) {
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


