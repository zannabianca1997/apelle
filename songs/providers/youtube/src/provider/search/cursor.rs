use base64::Engine as _;
use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize, de::Error as _, ser::Error as _};

#[derive(Debug, Clone, Encode, Decode)]
pub struct Cursor {
    pub(crate) page: Option<String>,
    pub(crate) offset: i32,
}
impl Cursor {
    pub(crate) fn new() -> Self {
        Self {
            page: None,
            offset: 0,
        }
    }
}

// Encoding and decodind as byte strings

impl Serialize for Cursor {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        base64::engine::general_purpose::STANDARD_NO_PAD
            .encode(
                bincode::encode_to_vec(self, bincode::config::standard())
                    .map_err(|e| S::Error::custom(e.to_string()))?,
            )
            .serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Cursor {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(bincode::decode_from_slice(
            &base64::engine::general_purpose::STANDARD_NO_PAD
                .decode(String::deserialize(deserializer)?)
                .map_err(|e| D::Error::custom(e.to_string()))?,
            bincode::config::standard(),
        )
        .map_err(|e| D::Error::custom(e.to_string()))?
        .0)
    }
}
