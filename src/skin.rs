use {
    crate::{
        consts::*,
    },
    crossterm::{
        style::{
            Color,
            SetBackgroundColor,
        },
    },
    termimad::{
        gray,
        ansi,
    },
};

/// skin for a foreground element
pub struct FgSkin {
    pub color: Color,
    pub chr: char,
}
impl FgSkin {
    pub fn new(chr: char, color: Color) -> Self {
        Self { chr, color }
    }
}

pub struct Skin {
    pub cell_bg: [Color; 4],
    pub lapin: FgSkin,
    pub fox: FgSkin,
}

impl Skin {
    pub fn new() -> Self {
        let cell_bg = [
            gray(2),  // VOID
            ansi(59), // WALL
            ansi(22), // FOREST
            ansi(25), // WATER
        ];
        Self {
            cell_bg,
            //lapin: FgSkin::new('◆', gray(13)),
            lapin: FgSkin::new('▮', gray(16)),
            fox: FgSkin::new('█', ansi(166)),
            // full:'█'
        }
    }
    pub fn bg_command(&self, cell: Cell) -> SetBackgroundColor {
        SetBackgroundColor(self.cell_bg[cell as usize])
    }
    pub fn bg(&self, cell: Cell) -> Color {
        self.cell_bg[cell as usize]
    }


}