use {
    anyhow::Result,
    crate::{
        level::Level,
    },
};


mod signature;
mod win_file;

pub use signature::Signature;
pub use win_file::WinFile;

pub fn save_win(level: &Level) -> Result<()> {
    let signature = Signature::new(level)?;
    let mut wf = WinFile::load()?;
    if wf.has_win(&signature) {
        debug!("not the first win of this level");
    } else {
        debug!("it's the first win of this level");
        wf.add_win(signature);
        wf.write()?;
    }
    Ok(())
}
