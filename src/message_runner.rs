use {
    crate::{
        app::AppState,
        command::Command,
        io::W,
        screen::Screen,
        status::Status,
        task_sync::*,
    },
    anyhow::Result,
    crossterm::{
        event::{
            KeyEvent,
        },
    },
    minimad::{
        Composite,
    },
    termimad::{
        Event,
    },
};

fn handle_event(
    event: Event,
) -> Result<Option<AppState>> {
    Ok(match event {
        Event::Key(KeyEvent { code, .. }) => {
            match Command::from(code) {
                Some(Command::Move(_)) => None,
                _ => Some(AppState::Quit),
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
    good: bool,
    dam: &mut Dam,
) -> Result<AppState> {
    let screen = Screen::new()?;
    debug!("good={}", good);
    Status::from(Composite::from_inline(&message), !good).display(w, &screen)?;
    loop {
        let event = dam.next_event().unwrap();
        dam.unblock();
        if let Some(state) = handle_event(event)? {
            return Ok(state);
        }
    }
}

