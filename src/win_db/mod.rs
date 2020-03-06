use {
    anyhow::Result,
};


mod signature;
mod win_file;

pub use {
    signature::Signature,
    win_file::WinFile,
};

/// save the information that the current user solved the
/// level. It's based on the level signature so stays valid
/// if a level file is moved or is included in another campaign
/// but is invalidated when the level is changed.
pub fn save_win(level_signature: &Signature) -> Result<()> {
    let mut wf = WinFile::load()?;
    if wf.has_win(level_signature) {
        debug!("not the first win of this level");
    } else {
        debug!("it's the first win of this level");
        wf.add_win(level_signature.clone());
        wf.write()?;
    }
    Ok(())
}
