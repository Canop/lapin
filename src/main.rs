#[macro_use]
extern crate log;

use {
    anyhow::Result,
    crossterm::{
        cursor,
        terminal::{
            self,
            EnterAlternateScreen,
            LeaveAlternateScreen,
        },
        QueueableCommand,
    },
    lapin::{
        app,
        task_sync::*,
    },
    log::LevelFilter,
    simplelog,
    std::{
        env,
        fs::File,
        io::Write,
        str::FromStr,
    },
};

/// configure the application log according to env variable.
///
/// There's no log unless the LAP_LOG environment variable is set to
///  a valid log level (trace, debug, info, warn, error, off)
/// Example:
///      LAP_LOG=info broot
fn configure_log() {
    let level = env::var("LAP_LOG").unwrap_or_else(|_| "off".to_string());
    if level == "off" {
        return;
    }
    if let Ok(level) = LevelFilter::from_str(&level) {
        simplelog::WriteLogger::init(
            level,
            simplelog::Config::default(),
            File::create("dev.log").expect("Log file can't be created"),
        )
        .expect("log initialization failed");
        info!(
            "Starting Lapin v{} with log level {}",
            env!("CARGO_PKG_VERSION"),
            level
        );
    }
}

fn main() -> Result<()> {
    configure_log();
    let mut w = std::io::stderr();
    w.queue(EnterAlternateScreen)?;
    w.queue(cursor::Hide)?; // hiding the cursor
    terminal::enable_raw_mode()?;
    let mut dam = Dam::new()?;
    app::run(&mut w, &mut dam);
    dam.kill();
    terminal::disable_raw_mode()?;
    w.queue(cursor::Show)?;
    w.queue(LeaveAlternateScreen)?;
    w.flush()?;
    Ok(())
}
