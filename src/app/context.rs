
use {
    super::Fromage,
    crate::{
        display::{
            mad_skin,
            Skin,
            W,
        },
    },
    super::{
        Dam,
    },
    termimad::MadSkin,
};


/// The context of execution passed to the currently ran app state
pub struct Context<'c> {

    /// the dam which releases events and stops long tasks
    pub dam: &'c mut Dam,

    /// the writer where the app state must draw
    pub w: &'c mut W,

    /// The Lapin Skin
    pub skin: Skin,

    /// The Termimad skin used for markdown rendering
    pub mad_skin: MadSkin,
}

impl<'c> Context<'c> {
    pub fn new(
        dam: &'c mut Dam,
        w: &'c mut W,
        fromage: &Fromage,
    ) -> Self {
        let skin = Skin::new(fromage.color_blind());
        let mad_skin = mad_skin::make(&skin);
        Self {
            dam,
            w,
            skin,
            mad_skin,
        }
    }
}
