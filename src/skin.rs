use {
    crate::{
        consts::*,
        item::*,
    },
    crossterm::{
        style::{
            Attributes,
            ContentStyle,
            Color,
            PrintStyledContent,
            SetForegroundColor,
            SetBackgroundColor,
        },
    },
    termimad::{
        ansi,
        gray,
        rgb,
    },
};

/// skin for a foreground element
#[derive(Debug, Clone, Copy)]
pub struct FgSkin {
    pub color: Color,
    pub chr: char,
}
impl FgSkin {
    pub fn new(chr: char, color: Color) -> Self {
        Self { chr, color }
    }
    pub fn fg_command(&self) -> SetForegroundColor {
        SetForegroundColor(self.color)
    }
}

#[derive(Debug)]
pub struct Skin {
    pub cell_bg: [Color; 4],
    pub lapin: FgSkin,
    pub fox: FgSkin,
    pub wolf: FgSkin,
    pub knight: FgSkin,
    pub carrot: FgSkin,
    pub hunter: FgSkin,
    pub aiming_up: char,
    pub aiming_right: char,
    pub aiming_down: char,
    pub aiming_left: char,
    pub fire_horizontal: FgSkin,
    pub fire_vertical: FgSkin,
}

impl Skin {
    pub fn new() -> Self {
        let cell_bg = [
            //rgb(49, 41, 34),  // FIELD
            rgb(27, 23, 19),  // FIELD
            ansi(59), // WALL
            ansi(22), // FOREST
            ansi(25), // WATER
        ];
        Self {
            cell_bg,
            lapin: FgSkin::new('▮', gray(16)),
            fox: FgSkin::new('█', ansi(166)),
            knight: FgSkin::new('█', ansi(206)),
            wolf: FgSkin::new('█', gray(0)),
            carrot: FgSkin::new('⬩', ansi(172)),
            hunter: FgSkin::new('█', ansi(58)),
            aiming_up: '▴',
            aiming_right: '▸',
            aiming_down: '▾',
            aiming_left: '◂',
            fire_horizontal: FgSkin::new('―', Color::White),
            fire_vertical: FgSkin::new('│', Color::White),
        }
    }
    pub fn bg_command(&self, cell: Cell) -> SetBackgroundColor {
        SetBackgroundColor(self.cell_bg[cell as usize])
    }
    pub fn bg(&self, cell: Cell) -> Color {
        self.cell_bg[cell as usize]
    }
    pub fn styled_char(&self, item: Item, cell: Cell) -> PrintStyledContent<char> {
        let fg_skin = item.kind.skin(self);
        let cs = ContentStyle {
            foreground_color: Some(fg_skin.color),
            background_color: Some(self.bg(cell)),
            attributes: Attributes::default(),
        };
        PrintStyledContent(cs.apply(fg_skin.chr))
    }

}
