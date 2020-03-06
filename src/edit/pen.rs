use {
    crate::{
        level::Level,
        pos::*,
        core::Terrain,
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
            ink: Ink::Terrain(if level.default_terrain==Terrain::Mud {
                Terrain::Stone
            } else {
                Terrain::Mud
            }),
            shape_start: None,
        }
    }

    /// Maybe change the state of the pen and return the drawing
    /// action which should be applied to board, if any.
    pub fn click(
        &mut self,
        click_pos: Pos,
        is_control_click: bool,
    ) -> Option<DrawingAction> {
        match self.shape {
            PenShape::Dot => {
                Some(DrawingAction::DotInk(self.ink, click_pos))
            }
            PenShape::Line => {
                if let Some(start) = self.shape_start {
                    let action = if is_control_click {
                        DrawingAction::CompassLineInk(self.ink, start, click_pos)
                    } else {
                        DrawingAction::LineInk(self.ink, start, click_pos)
                    };
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
    /// return the help to display depending on the state,
    /// if any.
    pub fn status_help(&self) -> Option<String> {
        match self.shape {
            PenShape::Line if self.shape_start.is_some() => {
                Some("Click again to draw a line - with the *ctrl* key down to force compass directions".to_string())
            }
            PenShape::Rect if self.shape_start.is_some() => {
                Some("Click again to draw a rectangle".to_string())
            }
            _ => None,
        }
    }

}





