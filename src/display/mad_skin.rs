use {
    crate::{
        core::Terrain,
    },
    crossterm::{
        style::Color::*,
    },
    minimad::{
        Compound,
    },
    super::Skin,
    termimad::{
        ansi,
        gray,
        Alignment,
        MadSkin,
    },
};

/// make a mad skin which may be used in termimad views.
/// Include direct replacements for Lapin characters.
///
// TODO don't recreate every time ?
pub fn make(skin: &Skin) -> MadSkin {
    let mut ms = MadSkin::default();
    let bg = Terrain::Mud.bg(skin);
    ms.set_headers_fg(AnsiValue(178));
    ms.paragraph.align = Alignment::Center;
    let mut lapin = skin.lapin.clone();
    lapin.set_bg(bg);
    let mut carrot = skin.carrot.clone();
    carrot.set_bg(bg);
    let mut wine = skin.wine.clone();
    wine.set_bg(bg);
    ms.special_chars.insert(Compound::raw_str("H").code(), skin.hunter.clone());
    ms.special_chars.insert(Compound::raw_str("K").code(), skin.knight.clone());
    ms.special_chars.insert(Compound::raw_str("L").code(), lapin);
    ms.special_chars.insert(Compound::raw_str("F").code(), skin.fox.clone());
    ms.special_chars.insert(Compound::raw_str("S").code(), skin.sheep.clone());
    ms.special_chars.insert(Compound::raw_str("W").code(), skin.wolf.clone());
    ms.special_chars.insert(Compound::raw_str("g").code(), Terrain::Grass.bg_as_styled_char(skin));
    ms.special_chars.insert(Compound::raw_str("s").code(), Terrain::Sand.bg_as_styled_char(skin));
    ms.special_chars.insert(Compound::raw_str("c").code(), carrot);
    ms.special_chars.insert(Compound::raw_str("w").code(), wine);
    ms.italic.set_fg(AnsiValue(178));
    ms.scrollbar.thumb.set_fg(AnsiValue(178));
    ms.inline_code.set_bg(ansi(22));
    ms.set_global_bg(bg);

    // style for selected line
    ms.headers[2] = Default::default();
    ms.headers[2].align = Alignment::Center;
    ms.headers[2].set_bg(ansi(24));

    // style for unselectable line
    ms.headers[3] = Default::default();
    ms.headers[3].align = Alignment::Center;
    ms.headers[3].set_fg(gray(9));
    ms.headers[3].set_bg(bg);

    ms
}
