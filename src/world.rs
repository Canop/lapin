use {
    crate::{
        board::Board,
        path,
        pos::*,
    },
};

/// what the world plays in a non-player turn.
/// Arrays here must be consistent with the board.
#[derive(Debug)]
pub struct WorldMove {
    pub fox_moves: Vec<Option<Dir>>,
    pub knight_moves: Vec<Option<Dir>>,
}

pub fn play(board: &Board) -> WorldMove {
    // Fox moves
    let mut path_finder = path::PathFinder::new(board);
    let fox_moves = board.foxes.iter()
        .map(|fox|
            if fox.pos == board.lapin.pos {
                None // already on target
            } else if Pos::mh_distance(fox.pos, board.lapin.pos) == 1 {
                // may be a diagonal move, only valid for the attack
                fox.pos.dir_to(board.lapin.pos)
            } else if let Some(path) = path_finder.find(fox.pos, board.lapin.pos) {
                if let Some(&first_pos) = path.get(0) {
                    path_finder.reserve(first_pos);
                    fox.pos.dir_to(first_pos)
                } else {
                    None
                }
            } else {
                None
            }
        )
        .collect();

    // Knight moves
    let mut path_finder = path::PathFinder::new(board);
    path_finder.reserve(board.lapin.pos);
    let knight_targets: Vec<Pos> = board.foxes.iter()
        .map(|fox| fox.pos)
        .collect();
    let knight_moves = board.knights.iter()
        .map(|knight| {
            if let Some(path) = path_finder.shortest(knight.pos, &knight_targets) {
                if let Some(&first_pos) = path.get(0) {
                    path_finder.reserve(first_pos);
                    knight.pos.dir_to(first_pos)
                } else {
                    None
                }
            } else {
                None
            }
        }).collect();

    WorldMove {
        fox_moves,
        knight_moves,
    }
}
