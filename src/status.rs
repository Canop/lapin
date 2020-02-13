use {
    anyhow::Result,
    crate::{
        io::W,
        screen::Screen,
    },
    minimad::{
        Alignment,
        Composite,
    },
};

/// the status contains information written on the grey line
///  near the bottom of the screen
pub struct Status<'a> {
    message: Composite<'a>,
    error: bool, // is the current message an error?
}

impl<'a> Status<'a> {

    pub fn from_message(message: Composite<'a>) -> Status<'a> {
        Self {
            message,
            error: false,
        }
    }

    pub fn from(message: Composite<'a>, error: bool) -> Status<'a> {
        Self {
            message,
            error,
        }
    }

    pub fn display(self, w: &mut W, screen: &Screen) -> Result<()> {
        let y = screen.height - 1;
        screen.goto_clear(w, 0, y)?;
        let x = 0;
        screen.goto(w, x as u16, y)?;
        let skin = if self.error {
            &screen.skin.error_status
        } else {
            &screen.skin.normal_status
        };
        skin.write_inline_on(w, " ")?;
        let remaining_width = screen.width as usize - x - 1;
        skin.write_composite_fill(w, self.message, remaining_width, Alignment::Left)?;
        screen.clear_line(w)
    }
}

