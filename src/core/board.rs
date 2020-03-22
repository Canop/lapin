use {
    anyhow::Result,
    crate::{
        persist::Level,
        pos::*,
    },
    std::{
        ops::{
            Range,
        },
    },
    super::*,
};

static GAME_AREA: PosArea = PosArea::new(-1000..1000, -1000..1000);

/// the game state
pub struct Board {
    pub name: String,
    pub area: PosArea,
    pub terrains: PosMap<Terrain>,
    pub actors: ActorMap,
    pub items: OptionPosMap<Item>,
    pub current_player: Player, // whose turn it is
}

impl From<&Level> for Board {
    fn from(level: &Level) -> Self {
        let mut board = Board::new(
            level.name.clone(),
            PosArea::empty(),
            level.default_terrain,
        );
        board.reset_to(level);
        board
    }
}

impl Board {

    pub fn new(
        name: String,
        area: PosArea,
        default_terrain: Terrain,
    ) -> Self {
        let terrains = PosMap::new(area.clone(), default_terrain);
        let actors = ActorMap::from(area.clone());
        let items = OptionPosMap::new(area.clone(), None);
        Self {
            name,
            area,
            terrains,
            actors,
            items,
            current_player: Player::Lapin,
        }
    }

    pub fn default_terrain(&self) -> Terrain {
        self.terrains.default
    }

    pub fn reset_to(&mut self, level: &Level) {
        let pos_distribution = PosDistribution::from(
            level.terrains.iter()
                .filter(|lc| lc.v != level.default_terrain)
                .map(|&lc| lc.pos)
        )
        .unwrap_or_default();
        self.terrains = PosMap::new(pos_distribution.area.clone(), level.default_terrain);
        for &lc in level.terrains.iter() {
            self.terrains.set_lc(lc);
        }
        self.actors = ActorMap::new(pos_distribution.area.clone(), level.actors.clone());
        self.items = OptionPosMap::new(pos_distribution.area.clone(), None);
        for lc in &level.items {
            self.items.set_some(lc.pos, lc.v);
        }
    }

    pub fn lapin_pos(&self) -> Pos {
        self.actors.lapin().pos
    }

    pub fn add_actor_in(&mut self, kind: ActorKind, x: Int, y: Int) -> Result<ActorRef> {
        self.actors.add(Actor::new(kind, x, y))
    }
    pub fn add_item_in(&mut self, kind: ItemKind, x: Int, y: Int) {
        self.items.set_some(Pos::new(x, y), Item { kind });
    }
    pub fn set(&mut self, pos: Pos, terrain: Terrain) {
        self.terrains.set(pos, terrain);
    }
    pub fn set_range(&mut self, rx: Range<Int>, ry: Range<Int>, terrain: Terrain) {
        for x in rx {
            for y in ry.clone() {
                self.set(Pos::new(x, y), terrain);
            }
        }
    }
    pub fn set_h_line(&mut self, rx: Range<Int>, y: Int, terrain: Terrain) {
        self.set_range(rx, y..y+1, terrain);
    }
    pub fn set_v_line(&mut self, x: Int, ry: Range<Int>, terrain: Terrain) {
        self.set_range(x..x+1, ry, terrain);
    }
    pub fn get(&self, pos: Pos) -> Terrain {
        self.terrains.get(pos)
    }

    pub fn apply_player_move(&mut self, dir: Dir) -> MoveResult {
        if self.current_player != Player::Lapin {
            return MoveResult::Invalid;
        }
        let mut end_turn = true;
        let pos = self.lapin_pos().in_dir(dir);
        if !GAME_AREA.contains(pos) {
            warn!("Lapin is too far!");
            return MoveResult::Invalid;
        }
        if !self.actors.lapin().can_enter(self.get(pos)) {
            debug!("can't go there");
            return MoveResult::Invalid
        }
        if let Some(actor) = self.actors.by_pos(pos) {
            if actor.runs_after(self.actors.lapin()) {
                self.current_player = Player::None;
                // in order to move the lapin, we must mark
                // it dead first (or an error would be thrown)
                self.actors.state_by_id_mut(0).dead = true;
                self.actors.move_by_id_to_pos(0, pos).unwrap();
                return MoveResult::PlayerLose(format!(
                    "You have been eaten by a *{:?}*.", actor.kind
                ));
            } else {
                return MoveResult::Invalid;
            }
        }
        self.actors.move_lapin_to(pos);
        if self.get(pos) == Terrain::Grass {
            self.current_player = Player::None;
            return MoveResult::PlayerWin(
                "You're on the grass.".to_string()
            );
        }
        if let Some(Item{ kind: ItemKind::Carrot }) = self.items.get(pos) {
            self.items.remove(pos);
            info!("Lapin eats a carrot and replays");
            end_turn = false;
        }
        if end_turn {
            self.current_player = Player::World;
        }
        MoveResult::Ok
    }

    pub fn apply_world_move(&mut self, world_move: &mut WorldMove) -> MoveResult {
        let mut result = MoveResult::Ok;
        self.current_player = Player::Lapin;
        let mut kept_moves = Vec::new();
        for actor_move in &world_move.actor_moves {
            let actor_id = actor_move.actor_id;
            let actor = self.actors.by_id(actor_id);
            if actor.state.dead {
                debug!("move prevented because actor already dead");
                continue;
            }
            match actor_move.action {
                Action::Eats(dir, target_id) => {
                    let target_state = self.actors.state_by_id_mut(target_id);
                    if target_state.dead {
                        debug!("eating prevented because target already dead");
                        continue;
                    }
                    target_state.dead = true;
                    if target_id == 0 {
                        self.current_player = Player::None;
                        result = MoveResult::PlayerLose(
                            format!("You have been eaten by a *{:?}*.", self.actors.by_id(actor_id).kind)
                        );
                    }
                    if let Err(e) = self.actors.move_by_id_in_dir(actor_id, dir) {
                        debug!("{:?} can't eat in {:?} : {:?}", self.actors.by_id(actor_id).kind, dir, e);
                        continue;
                    }
                }
                Action::Fires(_, target_id) => {
                    if !self.actors.by_id(target_id).kind.is_immune_to_fire() {
                        let target_state = self.actors.state_by_id_mut(target_id);
                        if target_state.dead {
                            debug!("firing prevented because target already dead");
                            continue;
                        }
                        target_state.dead = true;
                        if target_id == 0 {
                            self.current_player = Player::None;
                            result = MoveResult::PlayerLose(
                                format!("You have been killed by a *{:?}*.", self.actors.by_id(actor_id).kind)
                            );
                        }
                    }
                }
                Action::Moves(dir) => {
                    let new_pos = self.actors.by_id(actor_id).pos.in_dir(dir);
                    match self.actors.move_by_id_to_pos(actor_id, new_pos) {
                        Err(e) => {
                            debug!("{:?} can't move in {:?}: {:?}", self.actors.by_id(actor_id).kind, dir, e);
                            continue;
                        }
                        Ok(()) => {
                            if self.actors.by_id(actor_id).kind.drinks_wine() {
                                if let Some(Item{kind:ItemKind::Wine}) = self.items.get(new_pos) {
                                    self.items.remove(new_pos);
                                    info!("hunter drinks some wine");
                                    self.actors.state_by_id_mut(actor_id).drunk = true;
                                }
                            }
                        }
                    }
                }
                Action::Aims(dir) => {
                    self.actors.state_by_id_mut(actor_id).aim = Some(dir);
                }
                Action::StopsAiming => {
                    self.actors.state_by_id_mut(actor_id).aim = None;
                }
            }
            kept_moves.push(*actor_move);
        }
        self.actors.remove_dead();
        world_move.actor_moves = kept_moves;
        result
    }
}
