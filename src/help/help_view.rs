
use {
    anyhow::Result,
    crate::{
        app::{
            Context,
            State,
            StateTransition,
        },
        display::{
            Layout,
            Screen,
            Status,
        },
        persist::Level,
        pos::*,
    },
    crossterm::{
        event::{
            KeyCode,
            KeyEvent,
        },
    },
    termimad::{
        Event,
        MadView,
    },
};

pub struct View {
    layout: Layout,
    markdown: &'static str,
}

impl View {
    pub fn new(
        markdown: &'static str,
        layout: Layout,
    ) -> Self {
        Self {
            markdown,
            layout,
        }
    }
}

impl State for View {

    fn label(&self) -> &'static str {
        "help"
    }

    fn run(
        &mut self,
        con: &mut Context,
    ) -> Result<StateTransition> {
        let mut screen = Screen::new(self.layout);
        let mut mad_view = MadView::from(
            self.markdown.to_string(),
            screen.areas.help.clone(),
            con.mad_skin.clone(),
        );
        loop {
            Status::from_message("Hit *esc* to close the help".to_string())
                .display(con, &screen)?;
            mad_view.write_on(con.w)?;
            let event = con.dam.next_event().unwrap();
            con.dam.unblock();
            match event {
                Event::Key(KeyEvent { code, .. }) => {
                    match code {
                        KeyCode::Up => {
                            mad_view.try_scroll_lines(-1);
                        }
                        KeyCode::Down => {
                            mad_view.try_scroll_lines(1);
                        }
                        KeyCode::PageUp => {
                            mad_view.try_scroll_pages(-1);
                        }
                        KeyCode::PageDown => {
                            mad_view.try_scroll_pages(1);
                        }
                        KeyCode::Char('q') => {
                            return Ok(StateTransition::Quit);
                        }
                        KeyCode::Esc => {
                            return Ok(StateTransition::Back);
                        }
                        _ => {
                            debug!("ignored code");
                        }
                    }
                }
                Event::Resize(width, height) => {
                    screen.set_terminal_size(width, height);
                    mad_view.resize(&screen.areas.help);
                }
                Event::Click(x, y, ..) => {
                    let sp = ScreenPos{ x, y };
                    debug!("click in sp={:?}", sp);
                }
                _ => {
                    debug!("ignored event: {:?}", event);
                }
            }
        }
    }

    fn get_level(
        &self,
        _level_idx: usize,
    ) -> Option<Level> {
        None
    }

}
