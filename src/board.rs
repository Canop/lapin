use {
    crate::{
        actor::*,
        command::*,
        consts::*,
        item::*,
        pos::*,
        pos_map::*,
        world::*,
    },
    fnv::FnvHashMap,
    std::{
        ops::{
            Range,
        },
    },
};

/// the game state
pub struct Board {
    pub area: PosArea,
    pub cells: PosMap<Cell>,
    pub actors: Vec<Actor>, // lapin always at index 0
    pub items: FnvHashMap<Pos, Item>,
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

    pub fn new(area: PosArea, default_cell: Cell) -> Self {
        let cells = PosMap::new(area, default_cell);
        let mut actors = Vec::new();
        actors.push(Actor::new(ActorKind::Lapin, 0, 0));
        let items = FnvHashMap::default();
        Self {
            area,
            cells,
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
            FIELD => true,
            FOREST => true,
            _ => false,
        }
    }

    pub fn set(&mut self, pos: Pos, cell: Cell) {
        self.cells.set(pos, cell);
    }
    pub fn set_range(&mut self, rx: Range<Int>, ry: Range<Int>, cell: Cell) {
        for x in rx {
            for y in ry.clone() {
                self.cells.set_xy(x, y, cell);
            }
        }
    }
    pub fn set_h_line(&mut self, rx: Range<Int>, y: Int, cell: Cell) {
        self.set_range(rx, y..y+1, cell);
    }
    pub fn set_v_line(&mut self, x: Int, ry: Range<Int>, cell: Cell) {
        self.set_range(x..x+1, ry, cell);
    }
    pub fn get(&self, pos: Pos) -> Cell {
        self.cells.get(pos)
    }

    /// return a pos_set with the positions of all actors preset
    pub fn actors_map(&self) -> PosSet {
        let mut actors_map = PosSet::from(self.area);
        for &actor in &self.actors {
            actors_map.insert(actor.pos);
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
        let mut actors_map = self.actors_map();
        for actor_move in world_move.actor_moves {
            let actor_id = actor_move.actor_id;
            actors_map.remove(self.actors[actor_id].pos);
            if let Some(target_id) = actor_move.target_id {
                // following test is only valid now
                if self.actors[target_id].kind.is_immune_to_fire() {
                    debug!("target is is_immune_to_fire");
                } else {
                    killed[target_id] = true;
                }
            }
            match actor_move.action {
                Action::Eats(dir) => {
                    let new_pos = self.actors[actor_id].pos.in_dir(dir);
                    self.actors[actor_id].pos = new_pos;
                }
                Action::Moves(dir) => {
                    let new_pos = self.actors[actor_id].pos.in_dir(dir);
                    if actors_map.has_key(new_pos) {
                        debug!("move prevented because other actor present");
                    } else {
                        self.actors[actor_id].pos = new_pos;
                    }
                }
                Action::Aims(dir) => {
                    self.actors[actor_id].state = ActorState::Aiming(dir);
                }
                Action::StopsAiming => {
                    self.actors[actor_id].state = ActorState::Normal;
                }
                _ => { }
            }
            actors_map.insert(self.actors[actor_id].pos);
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
