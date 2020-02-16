
use {
    serde::{Serialize, Deserialize},
    super::*,
};

/// a glorified tuple of a pos and a value
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub struct Located<V>
    where V: Copy
{
    pub pos: Pos,
    pub v: V,
}

impl<V> Located<V>
    where V: Copy
{
    pub fn new(pos: Pos, v: V) -> Self {
        Self { pos, v }
    }
}
