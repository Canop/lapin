use {
    super::*,
    std::iter::IntoIterator,
};

#[derive(Debug)]
pub struct PosDistribution {
    pub center: Pos,
    pub area: PosArea,
}

impl Default for PosDistribution {
    fn default() -> Self {
        Self {
            center: Pos::new(0, 0),
            area: PosArea::empty(),
        }
    }
}

impl PosDistribution {
    pub fn from<I>(pos_iterable: I) -> Option<Self>
    where
        I: IntoIterator<Item = Pos>,
    {
        let mut pos_iter = pos_iterable.into_iter();
        pos_iter.next().map(|first_pos| {
            // we'll assume the Int type is big enough to not overflow here
            let mut sum = first_pos;
            let mut count = 1;
            let mut area = PosArea::from_pos(first_pos);
            for pos in pos_iter {
                sum.x += pos.x;
                sum.y += pos.y;
                count += 1;
                area.grow_to(pos);
            }
            let center = Pos::new(sum.x / count, sum.y / count);
            Self { center, area }
        })
    }
}
