use {
    crossterm::{
        style::{
            Attribute,
            Attributes,
            Color,
        },
    },
    termimad::{
        ansi,
        CompoundStyle,
        gray,
        MadSkin,
        StyledChar,
    },
};

pub struct Skin {
    // terrains
    pub mud: Color,
    pub stone: Color,
    pub grass: Color,
    pub water: Color,
    pub sand: Color,
    // actors
    pub fox: StyledChar,
    pub hunter: StyledChar,
    pub knight: StyledChar,
    pub lapin: StyledChar,
    pub sheep: StyledChar,
    pub wolf: StyledChar,
    pub dragon: StyledChar,
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
    pub hunter_fire_horizontal: StyledChar,
    pub hunter_fire_vertical: StyledChar,
    pub dragon_fire_horizontal: StyledChar,
    pub dragon_fire_vertical: StyledChar,
    // texts
    pub normal_status: MadSkin,
    pub error_status: MadSkin,
    pub editor: MadSkin,
    pub editor_circle: Color,
}

fn actor_char(
    letter: char,
    color: Color,
    color_blind: bool,
) -> StyledChar {
    if color_blind {
        let style = CompoundStyle::new(Some(color), None, Attribute::Bold.into());
        StyledChar::new(style, letter)
    } else {
        StyledChar::from_fg_char(color, '█')
    }
}

impl Skin {
    pub fn new(color_blind: bool) -> Self {
        Self {
            mud: gray(4),  // FIELD
            stone: ansi(59), // WALL
            grass: ansi(22), // GRASS
            water: ansi(25), // WATER
            sand: ansi(137), // SAND
            // actors
            fox: actor_char('F', ansi(166), color_blind),
            hunter: actor_char('H', ansi(58), color_blind),
            knight: actor_char('K', ansi(206), color_blind),
            lapin: StyledChar::from_fg_char(gray(16), '▮'),
            sheep: actor_char('S', gray(19), color_blind),
            wolf: actor_char('W', gray(0), color_blind),
            dragon: actor_char('D', ansi(51), color_blind),
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
            hunter_fire_horizontal: StyledChar::from_fg_char(Color::White, '―'),
            hunter_fire_vertical: StyledChar::from_fg_char(Color::White, '│'),
            dragon_fire_horizontal: StyledChar::from_fg_char(ansi(196), '―'),
            dragon_fire_vertical: StyledChar::from_fg_char(ansi(196), '│'),
            // texts
            normal_status: make_normal_status_mad_skin(),
            error_status: make_error_status_mad_skin(),
            editor: make_editor_mad_skin(),
            editor_circle: Color::White,
        }
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
    mad_skin.italic = CompoundStyle::new(Some(ansi(178)), None, Attributes::default());
    mad_skin.bold = CompoundStyle::new(Some(ansi(160)), None, Attribute::Bold.into());
    mad_skin
}

/// build a MadSkin which will be used in editor panels
fn make_editor_mad_skin() -> MadSkin {
    let mut mad_skin = MadSkin::default();
    mad_skin.paragraph.set_fg(gray(1));
    mad_skin.paragraph.set_bg(ansi(66));
    //mad_skin.paragraph.set_bg(ansi(137));
    //mad_skin.paragraph.set_bg(rgb(83, 72, 59));
    mad_skin.bold = CompoundStyle::new(Some(ansi(208)), None, Attribute::Bold.into());
    mad_skin
}
