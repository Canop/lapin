
/// what we get on applying a world or player move.
/// This will probably contain more in the future
#[derive(Debug)]
pub enum MoveResult {
    Ok, // RAS
    Invalid, // move does nothing,
    PlayerWin,
    PlayerLose,
}

#[derive(Debug, Clone, Copy)]
pub enum Player {
    Lapin, // played by a presumed human
    World, // the rest
}

