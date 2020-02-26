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
    },
    termimad::{
        ansi,
        CompoundStyle,
        gray,
        MadSkin,
        rgb,
        StyledChar,
    },
};

pub struct Skin {
    pub cell_bg: [Color; NB_TERRAINS as usize],
    // actors
    pub fox: StyledChar,
    pub hunter: StyledChar,
    pub knight: StyledChar,
    pub lapin: StyledChar,
    pub sheep: StyledChar,
    pub wolf: StyledChar,
    pub drunk_color: Color,
    // items
    pub carrot: StyledChar,
    pub wine: StyledChar,
    // special states
    pub aiming_up: char,
    pub aiming_right: char,
    pub aiming_down: char,
    pub aiming_left: char,
    // animations
    pub fire_horizontal: StyledChar,
    pub fire_vertical: StyledChar,
    // texts
    pub normal_status: MadSkin,
    pub error_status: MadSkin,
    pub editor: MadSkin,
    pub editor_circle: Color,
}

impl Default for Skin {
    fn default() -> Self {
        let cell_bg = [
            rgb(36, 27, 17),  // FIELD
            //rgb(51, 41, 29),  // FIELD
            //rgb(42, 32, 27),  // FIELD
            ansi(59), // WALL
            ansi(22), // GRASS
            ansi(25), // WATER
            rgb(83, 72, 59),  // SAND
            //ansi(137), // SAND
        ];
        Self {
            cell_bg,
            // actors
            fox: StyledChar::from_fg_char(ansi(166), '█'),
            hunter: StyledChar::from_fg_char(ansi(58), '█'),
            knight: StyledChar::from_fg_char(ansi(206), '█'),
            lapin: StyledChar::from_fg_char(gray(16), '▮'),
            sheep: StyledChar::from_fg_char(gray(19), '█'),
            wolf: StyledChar::from_fg_char(gray(0), '█'),
            drunk_color: ansi(160),
            // special states
            aiming_up: '▴',
            aiming_right: '▸',
            aiming_down: '▾',
            aiming_left: '◂',
            // items
            carrot: StyledChar::from_fg_char(ansi(172), '⬩'),
            wine: StyledChar::from_fg_char(ansi(160), '⬩'),
            // animations
            fire_horizontal: StyledChar::from_fg_char(Color::White, '―'),
            fire_vertical: StyledChar::from_fg_char(Color::White, '│'),
            // texts
            normal_status: make_normal_status_mad_skin(),
            error_status: make_error_status_mad_skin(),
            editor: make_editor_mad_skin(),
            editor_circle: Color::White,
        }
    }
}

impl Skin {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn bg_command(&self, cell: Cell) -> SetBackgroundColor {
        SetBackgroundColor(self.cell_bg[cell as usize])
    }
    pub fn bg(&self, cell: Cell) -> Color {
        self.cell_bg[cell as usize]
    }
    pub fn bg_as_styled_char(&self, cell: Cell) -> StyledChar {
        StyledChar::from_fg_char(self.bg(cell), '█')
    }
}

/// build a MadSkin which will be used to display the status
/// when there's no error
fn make_normal_status_mad_skin() -> MadSkin {
    let mut mad_skin = MadSkin::default();
    mad_skin.italic = CompoundStyle::new(Some(ansi(178)), None, Attributes::default());
    mad_skin.bold = CompoundStyle::new(Some(ansi(70)), None, Attribute::Bold.into());
    mad_skin
}

/// build a MadSkin which will be used to display the status
/// when there's a error
fn make_error_status_mad_skin() -> MadSkin {
    let mut mad_skin = MadSkin::default();
    mad_skin.bold = CompoundStyle::new(Some(ansi(160)), None, Attribute::Bold.into());
    mad_skin
}

/// build a MadSkin which will be used in editor panels
fn make_editor_mad_skin() -> MadSkin {
    let mut mad_skin = MadSkin::default();
    mad_skin.paragraph.set_fg(gray(1));
    mad_skin.paragraph.set_bg(ansi(137));
    //mad_skin.paragraph.set_bg(rgb(83, 72, 59));
    mad_skin.bold = CompoundStyle::new(Some(ansi(208)), None, Attribute::Bold.into());
    mad_skin
}
