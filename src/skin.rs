use {
    crate::{
        consts::*,
        item::*,
    },
    crossterm::{
        style::{
            Attribute,
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
        CompoundStyle,
        gray,
        MadSkin,
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
    pub fn fg_command(self) -> SetForegroundColor {
        SetForegroundColor(self.color)
    }
    pub fn styled_char_over(self, bg: Option<Color>) -> PrintStyledContent<char> {
        let cs = ContentStyle {
            foreground_color: Some(self.color),
            background_color: bg,
            attributes: Attributes::default(),
        };
        PrintStyledContent(cs.apply(self.chr))
    }
}

pub struct Skin {
    pub cell_bg: [Color; 4],
    // actors
    pub fox: FgSkin,
    pub hunter: FgSkin,
    pub hunter_drunk: FgSkin,
    pub knight: FgSkin,
    pub lapin: FgSkin,
    pub sheep: FgSkin,
    pub wolf: FgSkin,
    // items
    pub carrot: FgSkin,
    pub wine: FgSkin,
    // special states
    pub aiming_up: char,
    pub aiming_right: char,
    pub aiming_down: char,
    pub aiming_left: char,
    // animations
    pub fire_horizontal: FgSkin,
    pub fire_vertical: FgSkin,
    // texts
    pub normal_status: MadSkin,
    pub error_status: MadSkin,
    pub editor: MadSkin,
    pub editor_circle: Color,
}

impl Skin {
    pub fn new() -> Self {
        let cell_bg = [
            rgb(27, 23, 19),  // FIELD
            ansi(59), // WALL
            ansi(22), // GRASS
            ansi(25), // WATER
        ];
        Self {
            cell_bg,
            // actors
            fox: FgSkin::new('█', ansi(166)),
            hunter: FgSkin::new('█', ansi(58)),
            hunter_drunk: FgSkin::new('█', ansi(160)),
            knight: FgSkin::new('█', ansi(206)),
            lapin: FgSkin::new('▮', gray(16)),
            //sheep: FgSkin::new('█', ansi(230)),
            sheep: FgSkin::new('█', gray(19)),
            wolf: FgSkin::new('█', gray(0)),
            // special states
            aiming_up: '▴',
            aiming_right: '▸',
            aiming_down: '▾',
            aiming_left: '◂',
            // items
            carrot: FgSkin::new('⬩', ansi(172)),
            wine: FgSkin::new('⬩', ansi(160)),
            // animations
            fire_horizontal: FgSkin::new('―', Color::White),
            fire_vertical: FgSkin::new('│', Color::White),
            // texts
            normal_status: make_normal_status_mad_skin(),
            error_status: make_error_status_mad_skin(),
            editor: make_editor_mad_skin(),
            editor_circle: Color::White,
        }
    }
    pub fn bg_command(&self, cell: Cell) -> SetBackgroundColor {
        SetBackgroundColor(self.cell_bg[cell as usize])
    }
    pub fn bg(&self, cell: Cell) -> Color {
        self.cell_bg[cell as usize]
    }
    pub fn styled_item_char(&self, item: Item, cell: Cell) -> PrintStyledContent<char> {
        let fg_skin = item.kind.skin(self);
        let cs = ContentStyle {
            foreground_color: Some(fg_skin.color),
            background_color: Some(self.bg(cell)),
            attributes: Attributes::default(),
        };
        PrintStyledContent(cs.apply(fg_skin.chr))
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
    mad_skin.bold = CompoundStyle::new(Some(ansi(208)), None, Attribute::Bold.into());
    mad_skin
}
