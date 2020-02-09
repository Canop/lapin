use {
    crate::{
        app::AppState,
        command::Command,
        io::W,
        screen::Screen,
    },
    anyhow::Result,
    crossterm::{
        cursor,
        event::{
            self,
            Event,
            KeyEvent,
        },
        style::{
            Attribute,
            Color,
            ContentStyle,
            PrintStyledContent,
        },
        QueueableCommand,
    },
    std::io::Write,
    termimad::gray,
};

/// return the next state
pub fn run(w: &mut W, message: String) -> Result<AppState> {
    let screen = Screen::new()?;
    let cs = ContentStyle {
        foreground_color: Some(Color::Yellow),
        background_color: Some(gray(1)),
        attributes: Attribute::Bold.into(),
    };
    w.queue(cursor::MoveTo(10, screen.height-1))?;
    w.queue(PrintStyledContent(cs.apply(message)))?;
    w.flush()?;
    loop {
        if let Ok(Event::Key(KeyEvent { code, .. })) = event::read() {
            match Command::from(code) {
                Some(Command::Quit) => break,
                _ => { }
            }
        }
    }
    Ok(AppState::Quit)
}

