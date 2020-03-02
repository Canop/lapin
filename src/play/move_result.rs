
/// what we get on applying a world or player move.
/// This will probably contain more in the future
#[derive(Debug)]
pub enum MoveResult {
    Ok, // RAS
    Invalid, // move does nothing,
    PlayerWin(String),
    PlayerLose(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Player {
    Lapin, // played by a presumed human
    World, // the rest
    None, // game is probably finished
}

