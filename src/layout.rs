
use {
    termimad::Area,
};

/// contain all the areas, some of which empty depending
/// on the current app state, and other elements of
/// positionning depending on the screen dimensions.
#[derive(Debug)]
pub struct Areas {
    pub header: Area,
    pub board: Area,
    pub pen_panel: Area, // the ink pen_panel panel
    pub help: Area,
    pub status: Area,
    pub ink_margin: u16, // 0 or 1
}

/// layout contains the rules for defining the precise
/// areas for an app state
#[derive(Debug, Clone, Copy)]
pub struct Layout {
    pub header_height: u16,
    pub pen_panel_height: u16,
    pub status_height: u16,
}

impl Layout {
    /// container should usually be the whole screen.
    /// Note that the current implementation will panic if
    /// the screen isn't high enough.
    pub fn compute(self, con: &Area) -> Areas {
        let header = Area::new(
            con.left,
            con.top,
            con.width,
            self.header_height,
        );
        let status = Area::new(
            con.left,
            con.top + con.height - self.status_height,
            con.width,
            self.status_height,
        );
        let pen_panel = Area::new(
            con.left,
            status.top - self.pen_panel_height,
            con.width,
            self.pen_panel_height,
        );
        let ink_margin = if pen_panel.width > 85 { 1 } else { 0 };
        let board = Area::new(
            con.left,
            con.top + self.header_height,
            con.width,
            con.height - (
                self.header_height + self.pen_panel_height + self.status_height
            ),
        );
        let help = Area::new(
            con.left,
            con.top,
            con.width,
            con.height - self.status_height,
        );
        Areas {
            header,
            board,
            pen_panel,
            help,
            status,
            ink_margin,
        }
    }
}


