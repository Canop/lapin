
use {
    anyhow::Result,
    serde::Serialize,
    std::{
        fs::File,
        io::Write,
        path::Path,
    },
    super::*,
};



/// write an object (typically a Level) to a file
/// The real path may be different from the passed one if
/// a specific file format is requested
pub fn write_file<T>(
    val: &T,
    suggested_path: &Path,
    requested_format: Option<SerdeFormat>,
    pretty: bool,
) -> Result<()>
where
    T: Serialize,
{
    let format = requested_format
        .or(suggested_path.extension()
            .and_then(|os| os.to_str())
            .and_then(|ext| SerdeFormat::from_key(ext))
        )
        .unwrap_or_default();
    let path = suggested_path.with_extension(format.key());
    let mut file = File::create(path)?;
    write(&mut file, val, format, pretty)
}

/// write an object (typically a Level) to a stream
pub fn write<W: ?Sized, T>(
    w: &mut W,
    val: &T,
    format: SerdeFormat,
    pretty: bool,
) -> Result<()>
where
    W: Write,
    T: Serialize,
{
    match format {
        SerdeFormat::Json => {
            let serialized = if pretty {
                serde_json::to_string_pretty(val)
            } else {
                serde_json::to_string(val)
            }?;
            write!(w, "{}", serialized)?;
        }
        SerdeFormat::MessagePack => {
            rmp_serde::encode::write(w, val)?;
        }
    }
    Ok(())
}

