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
    termimad::{
        Event,
        EventSource,
        gray,
    },
};

fn handle_event(
    event: Event,
) -> Result<Option<AppState>> {
    Ok(match event {
        Event::Key(KeyEvent { code, .. }) => {
            match Command::from(code) {
                Some(Command::Move(_)) => None,
                _ => {
                    Some(AppState::Quit)
                }
            }
        }
        _ => {
            debug!("ignored event: {:?}", event);
            None
        }
    })
}

/// return the next state
pub fn run(
    w: &mut W,
    message: String,
    event_source: &EventSource,
) -> Result<AppState> {
    let screen = Screen::new()?;
    let cs = ContentStyle {
        foreground_color: Some(Color::Yellow),
        background_color: Some(gray(1)),
        attributes: Attribute::Bold.into(),
    };
    w.queue(cursor::MoveTo(10, screen.height-2))?;
    w.queue(PrintStyledContent(cs.apply(message)))?;
    w.flush()?;
    let rx_events = event_source.receiver();
    loop {
        event_source.unblock(false); // bof...
        let e = rx_events.recv();
        match e {
            Ok(event) => {
                if let Some(state) = handle_event(event)? {
                    return Ok(state);
                }
            }
            Err(e) => {
                debug!("error in event channel : {:?}", e);
            }
        }
    }
    Ok(AppState::Quit)
}

