
use {
    termimad::Area,
};

/// contain all the areas, some of which empty depending
/// on the current app state
#[derive(Debug)]
pub struct Areas {
    pub board: Area,
    pub selector: Area, // the ink selector panel
    pub status: Area,
}

/// layout contains the rules for defining the precise
/// areas for an app state
#[derive(Debug, Clone, Copy)]
pub struct Layout {
    pub selector_height: u16,
    pub status_height: u16,
}

impl Layout {
    /// container should usually be the whole screen.
    /// Note that the current implementation will panic if
    /// the screen isn't high enough.
    pub fn compute(self, con: &Area) -> Areas {
        let status = Area::new(
            con.left,
            con.top + con.height - self.status_height,
            con.width,
            self.status_height,
        );
        let selector = Area::new(
            con.left,
            status.top - self.selector_height,
            con.width,
            self.selector_height,
        );
        let board = Area::new(
            con.left,
            con.top,
            con.width,
            selector.top - con.top,
        );
        Areas {
            board,
            selector,
            status,
        }
    }
}


