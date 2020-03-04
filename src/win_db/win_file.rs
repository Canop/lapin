
use {
    anyhow::Result,
    dirs,
    std::{
        fs::{
            self,
            File,
        },
        io::{
            self,
            BufRead,
            Write,
        },
        path::PathBuf,
    },
    super::{
        Signature,
    },
};


/// An implementation of the wins storage based
/// on just a file with one line per level hash.
/// We use the hexa version for readability.
pub struct WinFile {
    path: PathBuf,
    wins: Vec<Signature>,
}

impl WinFile {
    pub fn load() -> Result<Self> {
        let dir = dirs::data_local_dir().ok_or(anyhow!("No data local dir"))?;
        let path = dir.join("lapin/wins");
        let mut wins = Vec::new();
        if let Ok(file) = File::open(&path) {
            for line in io::BufReader::new(file).lines() {
                wins.push(line?.into());
            }
        } else {
            info!("no wins file found at {:?}", &path);
        }
        Ok(Self {
            path,
            wins,
        })
    }
    ///
    pub fn has_win(&self, signature: &Signature) -> bool {
        self.wins.contains(signature)
    }
    /// `has` should be checked before
    pub fn add_win(&mut self, signature: Signature) {
        self.wins.push(signature);
    }
    /// the ` load` should be done "just before" to prevent
    /// problems with lapin applications kept open a long time
    pub fn write(&self) -> Result<()> {
        fs::create_dir_all(&self.path.parent().unwrap())?;
        let mut file = File::create(&self.path)?;
        for s in &self.wins {
            file.write_all(s.as_str().as_bytes())?;
            file.write_all(&[b'\n'])?;
        }
        debug!("wrote file {:?}", &self.path);
        Ok(())
    }
}
