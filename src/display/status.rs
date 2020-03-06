use {
    anyhow::Result,
    crate::{
        app::Context,
    },
    crossterm::{
        style::ResetColor,
        QueueableCommand,
    },
    minimad::{
        Alignment,
        Composite,
    },
    super::*,
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

    pub fn display(
        &self,
        con: &mut Context,
        screen: &Screen,
    ) -> Result<()> {
        let area = &screen.areas.status;
        screen.goto_clear(con.w, 0, area.top)?;
        screen.goto(con.w, area.left, area.top)?;
        let skin = if self.error {
            &con.skin.error_status
        } else {
            &con.skin.normal_status
        };
        con.w.queue(ResetColor)?;
        skin.write_inline_on(con.w, " ")?;
        let remaining_width = (area.width - area.left - 1) as usize;
        let composite = Composite::from_inline(&self.message);
        skin.write_composite_fill(con.w, composite, remaining_width, Alignment::Left)?;
        screen.clear_line(con.w)
    }
}

