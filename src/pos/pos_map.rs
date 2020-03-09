use {
    fnv::{
        FnvHashMap,
    },
    serde::{Serialize, Deserialize},
    super::*,
};

/// A structure keeping pos indexed values.
/// It makes sense when most of them are in a known rect but
/// it allows outliers.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PosMap<V>
    where V: Copy
{
    area: PosArea,
    pub default: V,
    grid: Vec<V>,                   // items in the area
    outliers: FnvHashMap<Pos, V>,   // items out of area
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
    /// iterate over both the in-grid elements and the outliers
    pub fn iter(&self) -> impl Iterator<Item = Located<V>> + '_ {
        self.grid.iter()
            .enumerate()
            .map(move |(i, &v)| Located::new(self.pos_unchecked(i), v))
            .chain(
                self.outliers.iter()
                .map(move |(&pos, &v)| Located::new(pos, v))
            )
    }
    fn idx(&self, pos: Pos) -> Option<usize> {
        if self.area.contains(pos) {
            Some((self.area.width() * (pos.y-self.area.y.start) + (pos.x-self.area.x.start)) as usize)
        } else {
            None
        }
    }
    pub fn clear(&mut self) {
        for c in self.grid.iter_mut() {
            *c = self.default;
        }
        self.outliers.clear();
    }
    fn pos_unchecked(&self, idx: usize) -> Pos {
        let dy = idx as Int / self.area.width();
        let dx = idx as Int % self.area.width();
        Pos::new(dx + self.area.x.start, dy + self.area.y.start)
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
    pub fn unset(&mut self, pos: Pos) {
        if let Some(idx) = self.idx(pos) {
            unsafe {
                *self.grid.get_unchecked_mut(idx) = self.default;
            }
        } else {
            self.outliers.remove(&pos);
        }
    }
    pub fn set_lc(&mut self, lc: Located<V>) {
        self.set(lc.pos, lc.v);
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
    /// iterate over real values
    pub fn iter_some(&self) -> impl Iterator<Item = Located<V>> + '_ {
        self.iter()
            .filter(|lc| lc.v.is_some())
            .map(|lc| Located::new(lc.pos, lc.v.unwrap()))
    }
}

#[cfg(test)]
mod pos_map_tests {

    use super::*;
    use crate::{
        core::*,
    };

    #[test]
    fn test_idx() {
        let area = PosArea::new(-10..11, -100..151);
        let m: PosMap<usize> = PosMap::new(area, 0);
        let pos = Pos::new(-7, 54);
        assert_eq!(pos, m.pos_unchecked(m.idx(pos).unwrap()));
    }

    #[test]
    fn test_cell_map() {
        use Terrain::*;
        let area = PosArea::new(-10..11, -100..151);
        let mut cm: PosMap<Terrain> = PosMap::new(area.clone(), Stone);
        cm.set_xy(2, 3, Grass);
        cm.set_xy(-5, 3, Water);
        cm.set_xy(-15, 3, Grass);
        assert_eq!(cm.get_xy(0, 0), Stone);
        assert_eq!(cm.get_xy(-1000, 0), Stone);
        assert_eq!(cm.get_xy(2, 3), Grass);
        assert_eq!(cm.get_xy(-5, 3), Water);
        assert_eq!(cm.get_xy(-15, 3), Grass); // out of area
        let iter = cm.iter();
        assert_eq!(
            iter.count() as i32,
            area.width()*area.height() + 1, // +1 for the outlier
        );
    }

    #[test]
    fn test_actor_map() {
        let area = PosArea::new(-10..11, -100..151);
        let mut am = ActorPosMap::from(area);
        let p = Pos::new(-5, 25);
        let fox = Actor::new(ActorKind::Fox, p.x, p.y);
        am.set(p, Some(fox));
        assert_eq!(am.has_xy(0, 0), false);
        assert_eq!(am.has_key(p), true);
        assert_eq!(am.get(p).map(|a|a.kind), Some(ActorKind::Fox));
    }

}

