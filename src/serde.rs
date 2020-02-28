
use {
    anyhow::Result,
    serde::{Serialize, de::DeserializeOwned},
    std::{
        fs::File,
        io::Write,
        path::Path,
    },
};

/// Formats usable for writing (and reading) levels.
///
/// Right now only JSON and Message Pack seem to make sense.
/// If another one appears to be desirable I can put it behind
/// a feature flag.
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum SerdeFormat {
    Json,
    MessagePack,
}

impl SerdeFormat {
    pub fn key(self) -> &'static str {
        match self {
            Self::Json => "json",
            Self::MessagePack => "mpack",
        }
    }
    pub fn from_key(key: &str) -> Option<Self> {
        match key {
            "json" => Some(SerdeFormat::Json),
            "mpack" | "mp" => Some(SerdeFormat::MessagePack),
            _ => None,
        }
    }
}

impl Default for SerdeFormat {
    fn default() -> Self {
        SerdeFormat::MessagePack
    }
}


/// write an object (typically a Level) to a file
/// The real path may be different from the passed one if
/// a specific file format is requested
pub fn write_file<T>(
    val: &T,
    suggested_path: &Path,
    requested_format: Option<SerdeFormat>,
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
    write(&mut file, val, format)
}

/// write an object (typically a Level) to a stream
pub fn write<W: ?Sized, T>(
    w: &mut W,
    val: &T,
    format: SerdeFormat,
) -> Result<()>
where
    W: Write,
    T: Serialize,
{
    match format {
        SerdeFormat::Json => {
            let serialized = serde_json::to_string(val)?;
            write!(w, "{}", serialized)?;
        }
        SerdeFormat::MessagePack => {
            rmp_serde::encode::write(w, val)?;
        }
    }
    Ok(())
}

/// read an object (typically a Level) from a file,
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


// Study of different formats:
// sizes below for the test level as of 2020/02/28
//
// JSON
//      serde_json = "1.0"
//      plus: easy to read and hack
//      minus: huge (14k)
// let serialized = serde_json::to_string(val)?;
// write!(w, "{}", serialized)?;
//
// Ron
//      ron = "0.5"
//      plus: easy to read
//      minus: hugissime (23k if readable), lack of prettyness configuration and tools
// let mut pretty = ron::ser::PrettyConfig::default();
// pretty.separate_tuple_members = false;
// pretty.indentor = "\t".to_string();
// let serialized = ron::ser::to_string_pretty(val, pretty)?;
// write!(w, "{}", serialized)?;
//
// YAML
//      serde_yaml = "0.8"
//      plus: easy to read and hack
//      minus: huge (18K)
// let serialized = serde_yaml::to_string(val)?;
// write!(w, "{}", serialized)?;
//
// Bincode
//    bincode = "1.2"
//    plus: not very big (4.3k)
//    minus: non readable, no tool
// let serialized = bincode::serialize(val)?;
// w.write_all(&serialized)?;
//
// CBOR
//      serde_cbor = "0.11"
//      plus: nothing
//      minus: wasteful (8k) non readable, no tool
// serde_cbor::to_writer(w, val)?;
//
// MessagePack
//      rmp_serde = "0.14"
//      plus: compact (2.6K)
//      minus: non readable, no edit tool ?
// rmp_serde::encode::write(w, val)?;
//
