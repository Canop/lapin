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
    std::{
        ops::{
            Range,
        },
    },
};

static GAME_AREA: PosArea = PosArea::new(-1000..1000, -1000..1000);

/// the game state
pub struct Board {
    pub area: PosArea,
    pub cells: PosMap<Cell>,
    pub actors: Vec<Actor>, // Lapin always at index 0
    pub items: OptionPosMap<Item>,
    pub current_player: Player, // whose turn it is
    //pub grass_areas: Vec<PosArea>, // targets for the sheeps
    pub grass_cells: Vec<Pos>, // targets for the sheeps
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
        let cells = PosMap::new(area.clone(), default_cell);
        let mut actors = Vec::new();
        actors.push(Actor::new(ActorKind::Lapin, 0, 0));
        let items = OptionPosMap::new(area.clone(), None);
        //let grass_areas = Vec::new();
        let grass_cells = Vec::new();
        Self {
            area,
            cells,
            actors,
            items,
            current_player: Player::Lapin,
            //grass_areas,
            grass_cells,
        }
    }

    pub fn lapin_pos(&self) -> Pos {
        self.actors[0].pos
    }

    pub fn add_actor_in(&mut self, kind: ActorKind, x: Int, y: Int) {
        self.actors.push(Actor::new(kind, x, y));
    }
    pub fn add_item_in(&mut self, kind: ItemKind, x: Int, y: Int) {
        self.items.set_some(Pos::new(x, y), Item { kind });
    }

    // FIXME remove
    pub fn is_enterable(&self, pos: Pos) -> bool {
        match self.get(pos) {
            FIELD => true,
            GRASS => true,
            _ => false,
        }
    }

    /// sets the area as range and mark it as a goal for sheeps
    pub fn add_grass_area(&mut self, rx: Range<Int>, ry: Range<Int>) {
        //self.grass_areas.push(PosArea::new(rx.clone(), ry.clone()));
        self.set_range(rx, ry, GRASS);
    }
    pub fn set(&mut self, pos: Pos, cell: Cell) {
        self.cells.set(pos, cell);
        if cell==GRASS {
            self.grass_cells.push(pos);
        }
    }
    pub fn set_range(&mut self, rx: Range<Int>, ry: Range<Int>, cell: Cell) {
        for x in rx {
            for y in ry.clone() {
                self.set(Pos::new(x, y), cell);
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
    pub fn actor_pos_set(&self) -> PosSet {
        let mut actor_pos_set = PosSet::from(self.area.clone());
        for &actor in &self.actors {
            actor_pos_set.insert(actor.pos);
        }
        actor_pos_set
    }

    /// return a pos_map referencing all the actors
    pub fn actor_pos_map(&self) -> ActorPosMap {
        let mut actor_pos_map = ActorPosMap::from(self.area.clone());
        for &actor in &self.actors {
            actor_pos_map.set(actor.pos, Some(actor));
        }
        actor_pos_map
    }

    pub fn apply_player_move(&mut self, cmd: Command) -> MoveResult {
        match cmd {
            Command::Move(dir) => {
                let mut end_turn = true;
                let pos = self.lapin_pos().in_dir(dir);
                if !GAME_AREA.contains(pos) {
                    warn!("Lapin is too far!");
                    return MoveResult::Invalid;
                }
                for i in 1..self.actors.len() {
                    if self.actors[i].pos == pos {
                        debug!("the place is taken");
                        return MoveResult::Invalid;
                    }
                }
                if self.is_enterable(pos) {
                    self.actors[0].pos = pos;
                    if self.get(pos) == GRASS {
                        return MoveResult::PlayerWin;
                    }
                    if let Some(Item{kind:ItemKind::Carrot}) = self.items.get(pos) {
                        self.items.remove(pos);
                        info!("Lapin eats a carrot and replays");
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
        let mut actor_pos_set = self.actor_pos_set();
        for actor_move in world_move.actor_moves {
            let actor_id = actor_move.actor_id;
            actor_pos_set.remove(self.actors[actor_id].pos);
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
                    if actor_pos_set.has_key(new_pos) {
                        debug!("move prevented because other actor present");
                    } else {
                        self.actors[actor_id].pos = new_pos;
                        if self.actors[actor_id].kind.drinks_wine() {
                            if let Some(Item{kind:ItemKind::Wine}) = self.items.get(new_pos) {
                                self.items.remove(new_pos);
                                info!("hunter drinks some wine");
                                self.actors[actor_id].state.drunk = true;
                            }
                        }
                    }
                }
                Action::Aims(dir) => {
                    self.actors[actor_id].state.aim = Some(dir);
                }
                Action::StopsAiming => {
                    self.actors[actor_id].state.aim = None;
                }
                _ => { }
            }
            actor_pos_set.insert(self.actors[actor_id].pos);
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
