mod structs;
pub use structs::*;

use crate::fs::encodings::{Encode, Encodings};
mod conversions;

impl Encode for ServerStateVersion0 {
    fn serialize_to_bytes(
        &self,
        encoding: crate::fs::encodings::Encodings,
    ) -> Result<Vec<u8>, crate::fs::encodings::EncodingError> {
        Ok(match encoding {
            Encodings::PostCard => postcard::to_stdvec(self)?,
            Encodings::BitCode => bitcode::encode(self),
            Encodings::WinCode => wincode::serialize(self)?,
        })
    }
    fn deserialize_to_self(
        bytes: &[u8],
        encoding: Encodings,
    ) -> Result<Self, crate::fs::encodings::EncodingError> {
        Ok(match encoding {
            Encodings::PostCard => postcard::from_bytes(bytes)?,
            Encodings::BitCode => bitcode::decode(bytes)?,
            Encodings::WinCode => wincode::deserialize(bytes)?,
        })
    }
}
