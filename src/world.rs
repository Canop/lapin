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
}

pub fn play(board: &Board) -> WorldMove {
    let fox_moves = board.foxes.iter()
        .map(|fox|
            path::find(fox.pos, board.lapin.pos, board)
            .and_then(|path| fox.pos.first_dir(&path))
        )
        .collect();
    WorldMove {
        fox_moves,
    }
}
