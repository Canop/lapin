

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

pub static FORMATS: &[SerdeFormat] = &[
    SerdeFormat::Json,
    SerdeFormat::MessagePack,
];

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
            "mpack" => Some(SerdeFormat::MessagePack),
            _ => None,
        }
    }
}

impl Default for SerdeFormat {
    fn default() -> Self {
        SerdeFormat::MessagePack
    }
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
