
use {
    anyhow::Result,
    crate::{
        persist,
    },
    ripemd160::{Ripemd160, Digest},
    serde::{Serialize, Deserialize},
    std::{
        fmt::Write,
        hash::Hash,
    },
};

/// The level signature is the key to recognize a level whatever
/// the file path and be sure that it applies to a specific version
/// (i.e. if a level is changed, the signature change). Cryptography
/// protection doesn't matter. What matters is to not have
/// accidental collisions.
/// The signature is the hexa representation of the RIPEMD-160 hash
/// of the message pack serialization of the level.
#[derive(Debug, Clone, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub struct Signature(String);

impl Signature {

    pub fn new<T>(t: &T) -> Result<Self>
        where T: Serialize
    {
        let mut bytes: Vec<u8> = Vec::new();
        persist::write(&mut bytes, t, persist::SerdeFormat::MessagePack, false)?;
        let mut hasher = Ripemd160::new();
        hasher.input(bytes);
        let hashed = hasher.result();
        let mut hexa = String::with_capacity(hashed.len()*2);
        for b in hashed {
            write!(&mut hexa, "{:02X}", b)?;
        }
        Ok(Signature(hexa))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

}

impl From<String> for Signature {
    fn from(s: String) -> Self {
        Self(s)
    }
}

