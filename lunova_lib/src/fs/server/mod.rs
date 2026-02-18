#![allow(clippy::modulo_one)]
/// Server data versions
pub mod versions;

use std::hint::unlikely;

use mirl::{misc::EasyUnwrapUnchecked, prelude::*};
pub use versions::v0::ServerStateVersion0;

use crate::{
    fs::encodings::{Encode, EncodingError, Encodings},
    server::ServerState,
};
/// The latest existing server storage state
pub type LatestServerState = ServerStateVersion0;
/// An error type bc I'm lazy
pub type ConversionResultError<T> = Result<T, EncodingError>;

#[repr(u16)]
#[derive(Debug, PartialEq, Eq, Copy)]
#[enum_ext::enum_extend]
/// The raw server version
pub enum ServerVersionsRaw {
    /// The first version
    V0 = 0,
}
#[derive(Debug, PartialEq, Eq)]
#[enum_ext::enum_extend]
/// A list of all server versions
///
/// 0 is seen as an error state
pub enum ServerVersions {
    /// The first version
    V0(ServerStateVersion0) = 0,
}
impl ServerVersions {}

/// A trait for all server states to implement
pub const trait GetVersion {
    /// Get the version of the requested item
    ///
    /// Which should be Version{X} -> X + 1
    fn get_version() -> u16;
}

/// Safety that the struct always contains the latest and correct amount of serverstates
const _: () = {
    assert!(
        ServerVersionsRaw::count() as u16 == LatestServerState::get_version()
    );
    assert!(ServerVersionsRaw::count() == ServerVersions::count());
    // assert!(ServerState::)
};
/// When you already know the version of the bytes
///
/// # Errors
/// [`EncodingError`]
pub fn load_version(
    bytes: &[u8],
    encoding: Encodings,
    version: ServerVersionsRaw,
) -> ConversionResultError<ServerState> {
    Ok(match version {
        ServerVersionsRaw::V0 => {
            ServerState::from_value(load_v1(bytes, encoding)?)
        }
    })
}
/// Load [`ServerStateVersion0`]
/// 
/// # Errors
/// [`EncodingError`]
pub fn load_v1(
    bytes: &[u8],
    encoding: Encodings,
) -> ConversionResultError<ServerStateVersion0> {
    ServerStateVersion0::deserialize_to_self(bytes, encoding)
}
/// Load the server state from the raw bytes
/// 
/// # Errors
/// [`EncodingError`]
pub fn load_server_from_raw(
    bytes: &[u8],
    encoding: u8,
    version: u16,
) -> ConversionResultError<ServerState> {
    let Some(encoding) = Encodings::from_usize(encoding as usize) else {
        return Err(EncodingError::InvalidEncoding(encoding));
    };
    let max = ServerVersionsRaw::count() as u16;
    if unlikely(version == 0) {
        return Err(EncodingError::VersionError);
    }

    if unlikely(version > max) {
        return Err(EncodingError::VersionOutOfRange {
            given: version,
            latest_supported: max,
        });
    }
    load_version(
        bytes,
        encoding,
        *ServerVersionsRaw::ref_from_ordinal(version as usize - 1)
            .easy_unwrap_unchecked(),
    )
}
