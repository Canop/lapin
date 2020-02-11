use {
    crate::{
        actor::*,
        command::*,
        consts::*,
        item::*,
        pos::*,
        world::*,
    },
    std::{
        collections::{
            HashMap,
        },
        ops::{
            Range,
        },
    },
};

/// the game state
pub struct Board {
    pub width: Int,
    pub height: Int,
    pub default_cell: Cell,
    grid: Vec<Cell>,
    pub actors: Vec<Actor>, // lapin always at index 0
    pub items: HashMap<Pos, Item>,
    pub current_player: Player, // whose turn it is
}

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

impl Board {

    pub fn new(width: usize, height: usize) -> Self {
        let default_cell = VOID;
        let grid = vec![VOID; width * height];
        let mut actors = Vec::new();
        actors.push(Actor::new(ActorKind::Lapin, 0, 0));
        let items = HashMap::new();
        Self {
            width: width as Int,
            height: height as Int,
            default_cell,
            grid,
            actors,
            items,
            current_player: Player::Lapin,
        }
    }

    pub fn lapin_pos(&self) -> Pos {
        self.actors[0].pos
    }

    pub fn add_actor_in(&mut self, kind: ActorKind, x: Int, y: Int) {
        self.actors.push(Actor::new(kind, x, y));
    }
    pub fn add_item_in(&mut self, kind: ItemKind, x: Int, y: Int) {
        self.items.insert(Pos::new(x, y), Item { kind });
    }

    // FIXME remove
    pub fn is_enterable(&self, pos: Pos) -> bool {
        match self.get(pos) {
            VOID => true,
            FOREST => true,
            _ => false,
        }
    }

    pub fn grid_idx(&self, pos: Pos) -> Option<usize> {
        if pos.in_grid(self.width, self.height) {
            Some((self.width * pos.y + pos.x) as usize)
        } else {
            None
        }
    }
    pub fn set(&mut self, pos: Pos, cell: Cell) {
        if let Some(grid_idx) = self.grid_idx(pos) {
            self.grid[grid_idx] = cell;
        }
    }
    pub fn set_range(&mut self, rx: Range<Int>, ry: Range<Int>, cell: Cell) {
        for x in rx {
            for y in ry.clone() {
                self.set_at(x, y, cell);
            }
        }
    }
    pub fn set_h_line(&mut self, rx: Range<Int>, y: Int, cell: Cell) {
        self.set_range(rx, y..y+1, cell);
    }
    pub fn set_v_line(&mut self, x: Int, ry: Range<Int>, cell: Cell) {
        self.set_range(x..x+1, ry, cell);
    }
    pub fn set_at(&mut self, x: Int, y: Int, cell: Cell) {
        if let Some(grid_idx) = self.grid_idx(Pos{x, y}) {
            self.grid[grid_idx] = cell;
        }
    }
    pub fn get(&self, pos: Pos) -> Cell {
        match self.grid_idx(pos) {
            Some(grid_idx) => self.grid[grid_idx],
            None => self.default_cell,
        }
    }

    pub fn actors_map(&self) -> HashMap<Pos, Actor> {
        let mut actors_map = HashMap::new();
        for &actor in &self.actors {
            actors_map.insert(actor.pos, actor);
        }
        actors_map
    }

    pub fn apply_player_move(&mut self, cmd: Command) -> MoveResult {
        match cmd {
            Command::Move(dir) => {
                let mut end_turn = true;
                let pos = self.lapin_pos().in_dir(dir);
                if self.is_enterable(pos) {
                    self.actors[0].pos = pos;
                    if self.get(pos) == FOREST {
                        return MoveResult::PlayerWin;
                    }
                    if let Some(_item) = self.items.get(&pos) {
                        // right now there are only carrots
                        self.items.remove(&pos);
                        info!("Lapin eat a carrot");
                        end_turn = false;
                    }
                    for i in 1..self.actors.len() {
                        if self.actors[i].pos == pos {
                            return MoveResult::PlayerLose;
                        }
                    }
                    if end_turn {
                        self.current_player = Player::World;
                    }
                    MoveResult::Ok
                } else {
                    debug!("can't go there");
                    MoveResult::Invalid
                }
            }
            _ => {
                debug!("a pa capito");
                MoveResult::Invalid
            }
        }
    }

    pub fn apply_world_move(&mut self, world_move: WorldMove) -> MoveResult {
        let mut killed = vec![false; self.actors.len()];
        for actor_move in world_move.actor_moves {
            let actor_id = actor_move.actor_id;
            if let Some(target_id) = actor_move.target_id {
                // following test is only valid now
                if self.actors[target_id].kind.is_immune_to_fire() {
                    debug!("target is is_immune_to_fire");
                } else {
                    killed[target_id] = true;
                }
            }
            match actor_move.action {
                Action::Eats(dir) | Action::Moves(dir) => {
                    let new_pos = self.actors[actor_id].pos.in_dir(dir);
                    self.actors[actor_id].pos = new_pos;
                }
                Action::Aims(dir) => {
                    self.actors[actor_id].state = ActorState::Aiming(dir);
                }
                Action::StopsAiming => {
                    self.actors[actor_id].state = ActorState::Normal;
                }
                _ => { }
            }
        }
        self.current_player = Player::Lapin;
        if killed[0] {
            MoveResult::PlayerLose
        } else {
            let mut i = self.actors.len() - 1;
            while i > 0 {
                if killed[i] {
                    self.actors.remove(i);
                }
                i -= 1;
            }
            MoveResult::Ok
        }
    }
}
