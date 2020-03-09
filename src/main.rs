#[macro_use]
extern crate log;

use {
    anyhow::Result,
    argh,
    crossterm::{
        cursor,
        event::{DisableMouseCapture, EnableMouseCapture},
        terminal::{
            self,
            EnterAlternateScreen,
            LeaveAlternateScreen,
        },
        QueueableCommand,
    },
    lapin::{
        app::*,
        campaign,
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

/// execute a "campaign" command, which doesn't need the TUI
fn do_campaign_command(cc: &CampaignCommand) -> Result<()> {
    match &cc.sub {
        CampaignSubCommand::New(ncc) => campaign::create(ncc),
        CampaignSubCommand::Pack(pcc) => campaign::pack(pcc),
    }
}

/// execute all the commands which need the TUI
fn do_tui_command(fromage: Fromage) -> Result<()> {
    let mut w = std::io::stderr();
    w.queue(EnterAlternateScreen)?;
    w.queue(cursor::Hide)?; // hiding the cursor
    terminal::enable_raw_mode()?;
    w.queue(EnableMouseCapture)?;
    let mut dam = Dam::new()?;
    let mut app = App::new();
    let r = app.run(&mut w, &mut dam, fromage);
    w.queue(DisableMouseCapture)?;
    terminal::disable_raw_mode()?;
    w.queue(cursor::Show)?;
    w.queue(LeaveAlternateScreen)?;
    w.flush()?;
    r
}

fn main() {
    configure_log();
    let fromage: Fromage = argh::from_env();
    debug!("fromage: {:?}", &fromage);
    let r = match &fromage.command {
        Some(Command::Campaign(cc)) => do_campaign_command(cc),
        _ => do_tui_command(fromage),
    };
    if let Err(e) = r {
        warn!("Error: {:?}", e);
        println!("Error: {:?}", e);
    }
}
