use {
    crate::{board::Board, io::W},
    anyhow::Result,
    crossterm::{
        style::{
            Attributes, Color,
        },
    },
    termimad::{
        CompoundStyle,
    },
};

pub struct Editor {
    pub board: Board,
    //area: &Area,
}

impl Editor {
    pub fn new() -> Self {
        let board = Board::new(10, 10);
        Self { board }
    }
    pub fn write(&self, w: &mut W) -> Result<()> {
        let style = CompoundStyle::new(Some(Color::Blue), None, Attributes::default());

        style.queue_str(w, "Lapin!")?;
        Ok(())
    }
}
