
use {
    anyhow::Result,
    serde::{
        de::DeserializeOwned,
    },
    std::{
        fs::File,
        path::Path,
    },
    super::*,
};

pub fn read_bytes<T>(
    bytes: &[u8],
    format: SerdeFormat,
) -> Result<T>
where
    T: DeserializeOwned,
{
    Ok(match format {
        SerdeFormat::Json => {
            serde_json::from_slice(bytes)?
        }
        SerdeFormat::MessagePack => {
            rmp_serde::decode::from_slice(bytes)?
        }
    })
}


/// read an object from a file,
/// guessing the format from the file extension
pub fn read_file<T>(
    path: &Path,
) -> Result<T>
where
    T: DeserializeOwned,
{
    let format = path.extension()
        .and_then(|os| os.to_str())
        .and_then(|ext| SerdeFormat::from_key(ext))
        .unwrap_or_default();
    let file = File::open(path)?;
    debug!("read file {:?} with format {:?}", path, format);
    Ok(match format {
        SerdeFormat::Json => {
            serde_json::from_reader(file)?
        }
        SerdeFormat::MessagePack => {
            rmp_serde::decode::from_read(file)?
        }
    })
}
