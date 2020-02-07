use {
    crate::{
        consts::*,
    },
    crossterm::{
        style::{
            Attribute,
            Attributes,
            Color,
            SetBackgroundColor,
        },
        QueueableCommand,
    },
    termimad::{
        CompoundStyle,
        gray,
        ansi,
    },
};

pub struct Skin {
    pub cell_bg: [Color; 4],
    pub lapin_fg: Color,
}

impl Skin {
    pub fn new() -> Self {
        let cell_bg = [
            gray(0),  // VOID
            ansi(59), // WALL
            ansi(22), // FOREST
            ansi(45), // WATER
        ];
        Self {
            cell_bg,
            lapin_fg: gray(13),
        }
    }
    pub fn bg_command(&self, cell: Cell) -> SetBackgroundColor {
        SetBackgroundColor(self.cell_bg[cell as usize])
    }
    pub fn bg(&self, cell: Cell) -> Color {
        self.cell_bg[cell as usize]
    }


}
