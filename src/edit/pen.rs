use {
    crate::{
        consts::*,
        level::Level,
        pos::*,
    },
    super::{
        drawing_action::*,
        ink::*,
    },
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PenShape {
    Dot,
    Line,
    Rect,
}
pub static PEN_SHAPES: &[PenShape] = &[PenShape::Dot, PenShape::Line, PenShape::Rect];


/// defines what will happen on click on the board
#[derive(Debug, Clone, Copy)]
pub struct Pen {
    pub shape: PenShape,
    pub ink: Ink,
    shape_start: Option<Pos>,
}

impl Pen {

    pub fn new_for(level: &Level) -> Self {
        Self {
            shape: PenShape::Dot,
            ink: Ink::Terrain(if level.default_cell==FIELD { WALL } else { FIELD }),
            shape_start: None,
        }
    }

    /// Maybe change the state of the pen and return the drawing
    /// action which should be applied to board, if any.
    pub fn click(&mut self, click_pos: Pos) -> Option<DrawingAction> {
        match self.shape {
            PenShape::Dot => {
                Some(DrawingAction::DotInk(self.ink, click_pos))
            }
            PenShape::Line => {
                if let Some(start) = self.shape_start {
                    let action = DrawingAction::LineInk(self.ink, start, click_pos);
                    self.shape_start = None;
                    Some(action)
                } else {
                    self.shape_start = Some(click_pos);
                    None
                }
            }
            PenShape::Rect => {
                if let Some(start) = self.shape_start {
                    let action = DrawingAction::RectInk(self.ink, start, click_pos);
                    self.shape_start = None;
                    Some(action)
                } else {
                    self.shape_start = Some(click_pos);
                    None
                }
            }
        }
    }
    pub fn set_ink(&mut self, ink: Ink) {
        self.ink = ink;
        debug!("new pen ink: {:?}", self.ink);
    }
    pub fn set_shape(&mut self, shape: PenShape) {
        self.shape = shape;
        self.shape_start = None;
    }
    pub fn shape_started(&self) -> bool {
        self.shape_start.is_some()
    }

}





