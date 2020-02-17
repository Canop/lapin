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
pub struct Status {
    message: String, // markdown
    error: bool, // is the current message an error?
}

impl Status {

    pub fn from_message(message: String) -> Status {
        Self {
            message,
            error: false,
        }
    }

    pub fn from_error(message: String) -> Status {
        Self {
            message,
            error: true,
        }
    }

    pub fn from(message: String, error: bool) -> Status {
        Self {
            message,
            error,
        }
    }

    pub fn display(&self, w: &mut W, screen: &Screen) -> Result<()> {
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
        let composite = Composite::from_inline(&self.message);
        skin.write_composite_fill(w, composite, remaining_width, Alignment::Left)?;
        screen.clear_line(w)
    }
}

