use {
    crate::{
        display::Screen,
    },
    super::*,
};

/// do conversions between pos and screen_pos
#[derive(Debug, Clone, Copy)]
pub struct PosConverter {
    dec: Pos,
    dim: Pos,
}
impl PosConverter {
    pub fn from(center: Pos, screen: &Screen) -> Self {
        let dim = Pos {
            x: screen.areas.board.width as Int,
            y: screen.areas.board.height as Int,
        };
        let dec = Pos {
            x: dim.x / 2 - center.x,
            y: dim.y / 2 - center.y,
        };
        Self { dec, dim }
    }
    pub fn to_screen(&self, pos: Pos) -> Option<ScreenPos> {
        let x = pos.x + self.dec.x;
        let y = pos.y + self.dec.y;
        if x>=0 && y>=0 && x<self.dim.x && y<self.dim.y {
            Some(ScreenPos {
                x: x as u16,
                y: y as u16,
            })
        } else {
            None
        }
    }
    pub fn to_real(&self, sp: ScreenPos) -> Pos {
        let x = sp.x as Int - self.dec.x;
        let y = sp.y as Int - self.dec.y;
        Pos { x, y }
    }
}

