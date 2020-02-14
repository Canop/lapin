use {
    crate::{
        actor::*,
        pos::*,
    },
    fnv::{
        FnvHashMap,
    },
};

pub struct PosMap<V>
    where V: Copy
{
    area: PosArea,
    default: V,
    grid: Vec<V>, // for items in the area
    outliers: FnvHashMap<Pos, V>,
}
impl<V> PosMap<V>
    where V: Copy
{
    pub fn new(area: PosArea, default: V) -> Self {
        let grid = vec![default; (area.width()*area.height()) as usize];
        let outliers = FnvHashMap::default();
        Self {
            area,
            default,
            grid,
            outliers,
        }
    }
    fn idx(&self, pos: Pos) -> Option<usize> {
        if self.area.contains(pos) {
            Some((self.area.width() * (pos.y-self.area.y.start) + (pos.x-self.area.x.start)) as usize)
        } else {
            None
        }
    }
    pub fn get(&self, pos: Pos) -> V {
        if let Some(idx) = self.idx(pos) {
            unsafe {
                *self.grid.get_unchecked(idx)
            }
        } else if let Some(&v) = self.outliers.get(&pos) {
            v
        } else {
            self.default
        }
    }
    pub fn get_xy(&self, x: Int, y: Int) -> V {
        self.get(Pos::new(x, y))
    }
    pub fn set(&mut self, pos: Pos, v: V) {
        if let Some(idx) = self.idx(pos) {
            unsafe {
                *self.grid.get_unchecked_mut(idx) = v;
            }
        } else {
            self.outliers.insert(pos, v);
        }
    }
    pub fn set_xy(&mut self, x: Int, y: Int, v: V) {
        self.set(Pos::new(x, y), v);
    }
}

pub type PosSet = PosMap<bool>;
impl PosSet {
    pub fn from(area: PosArea) -> Self {
        PosMap::<bool>::new(area, false)
    }
    pub fn has_key(&self, pos: Pos) -> bool {
        self.get(pos)
    }
    pub fn insert(&mut self, pos: Pos) {
        self.set(pos, true);
    }
    pub fn remove(&mut self, pos: Pos) {
        self.set(pos, false);
    }
}

pub type OptionPosMap<V> = PosMap<Option<V>>;
impl<V> OptionPosMap<V>
    where V: Copy
{
    pub fn has_key(&self, pos: Pos) -> bool {
        self.get(pos).is_some()
    }
    pub fn remove(&mut self, pos: Pos) {
        self.set(pos, None);
    }
    pub fn has_xy(&self, x: Int, y: Int) -> bool {
        self.get_xy(x, y).is_some()
    }
    pub fn set_some(&mut self, pos: Pos, v: V) {
        self.set(pos, Some(v));
    }
}

// note that it's possible to insert an actor at
// a position other than its one (it can be for example
// his target)
pub type ActorPosMap = OptionPosMap<Actor>;
impl ActorPosMap {
    pub fn from(area: PosArea) -> Self {
        PosMap::<Option<Actor>>::new(area, None)
    }
    pub fn insert(&mut self, actor: Actor) {
        self.set(actor.pos, Some(actor));
    }
}

#[cfg(test)]
mod pos_map_tests {

    use super::*;
    use crate::actor::*;

    #[test]
    fn test_cell_map() {
        let area = PosArea {
            min: Pos::new(-10, -10),
            dim: Pos::new(100, 150),
        };
        let mut cm: PosMap<Cell> = PosMap::new(area, WALL);
        cm.set_xy(2, 3, GRASS);
        cm.set_xy(-5, 3, GRASS);
        cm.set_xy(-15, 3, GRASS);
        assert_eq!(cm.get_xy(0, 0), WALL);
        assert_eq!(cm.get_xy(-1000, 0), WALL);
        assert_eq!(cm.get_xy(2, 3), GRASS);
        assert_eq!(cm.get_xy(-5, 3), GRASS);
        assert_eq!(cm.get_xy(-15, 3), WALL); // out of area
    }

    #[test]
    fn test_actor_map() {
        let area = PosArea {
            min: Pos::new(-10, -10),
            dim: Pos::new(100, 150),
        };
        let mut am = ActorPosMap::from(area);
        let p = Pos::new(-5, 25);
        let fox = Actor::new(ActorKind::Fox, p.x, p.y);
        am.set(p, Some(fox));
        assert_eq!(am.has_xy(0, 0), false);
        assert_eq!(am.has_key(p), true);
        assert_eq!(am.get(p).map(|a|a.kind), Some(ActorKind::Fox));
    }

}

