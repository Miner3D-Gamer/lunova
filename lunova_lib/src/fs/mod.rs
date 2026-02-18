use mirl::prelude::FromPatch;

use crate::{
    fs::{
        encodings::{Encode, EncodingError, Encodings},
        server::{GetVersion, LatestServerState, load_server_from_raw},
    },
    server::ServerState,
};

/// Saving/Loading client data
pub mod client;
/// Saving/Loading server data
pub mod server;

/// The actual stuff turning data into bytes and bytes into data
pub mod encodings;
/// A special ID with which we are able to see if a binary is our file
pub const SPECIAL_ID: [char; 2] = [':', 'D'];
#[must_use]
/// Convert our special ID to a binary representation
pub fn special_id_to_bin() -> Vec<u8> {
    SPECIAL_ID
        .iter()
        .map(|x| unsafe { x.as_ascii_unchecked() }.to_u8())
        .collect()
}
/// Turn the server into a stable binary representation
///
/// # Errors
/// [`EncodingError`]
pub fn server_to_bin(
    state: ServerState,
    encoding: Encodings,
) -> Result<Vec<u8>, EncodingError> {
    let bytes =
        LatestServerState::from_value(state).serialize_to_bytes(encoding)?;

    let id =
        SPECIAL_ID.iter().map(|x| unsafe { x.as_ascii_unchecked() }.to_u8());
    let version = LatestServerState::get_version().to_le_bytes();

    let mut output = Vec::new();
    output.extend(id);
    output.extend(version);
    output.push(encoding as u8);
    output.extend(bytes);

    Ok(output)
}

/// # Return None
/// When the file ID is invalid.
///
/// # Errors
/// [`EncodingError`]
#[must_use]
pub fn server_from_bin(
    bytes: &[u8],
) -> Option<Result<ServerState, EncodingError>> {
    let id_end = SPECIAL_ID.len();
    let id = &bytes[..id_end];
    if special_id_to_bin() != id {
        return None;
    }

    let version =
        u16::from_le_bytes(bytes[id_end..id_end + 2].try_into().ok()?);
    let encoding = bytes[id_end + 2];
    let data = &bytes[id_end + 3..];

    Some(load_server_from_raw(data, encoding, version))
}
