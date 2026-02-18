#[derive(Debug, Copy, PartialEq, Eq)]
#[repr(u8)]
#[enum_ext::enum_extend]
/// How thing may be encoded
pub enum Encodings {
    /// A well maintained lib, the "default"
    /// A balance
    PostCard = 1,
    /// Fast and small, what else matters?
    BitCode = 2,
    /// No one invited him yet he still showed up for some reason
    /// Good for throughput, bad for actual size
    WinCode = 3,
}
// impl Encodings {

// }
#[derive(Debug)]
/// Errors for our supported encodings
pub enum EncodingError {
    /// The given bytes are from a newer version than supported
    VersionOutOfRange {
        /// The provided version
        given: u16,
        /// The latest version we support
        latest_supported: u16,
    },
    /// The version given is 0 which cannot be
    VersionError,
    /// The expected encoding does not exist
    InvalidEncoding(u8),
    /// [`postcard::Error`]
    PostCard(postcard::Error),
    /// [`bitcode::Error`]
    BitCode(bitcode::Error),
    /// [`WincodeReadWriteError`]
    WinCode(WincodeReadWriteError),
}

impl From<postcard::Error> for EncodingError {
    fn from(value: postcard::Error) -> Self {
        Self::PostCard(value)
    }
}
impl From<bitcode::Error> for EncodingError {
    fn from(value: bitcode::Error) -> Self {
        Self::BitCode(value)
    }
}
impl From<wincode::ReadError> for EncodingError {
    fn from(value: wincode::ReadError) -> Self {
        Self::WinCode(WincodeReadWriteError::Read(value))
    }
}
impl From<wincode::WriteError> for EncodingError {
    fn from(value: wincode::WriteError) -> Self {
        Self::WinCode(WincodeReadWriteError::Write(value))
    }
}
#[allow(missing_docs)]
#[derive(Debug)]
/// [`wincode::ReadError`]
/// [`wincode::WriteError`]
pub enum WincodeReadWriteError {
    Read(wincode::ReadError),
    Write(wincode::WriteError),
}

/// Convert an object to bytes and bytes to an object
pub trait Encode
where
    Self: std::marker::Sized,
{
    /// Turn self into bytes
    ///
    /// # Errors
    /// [`EncodingError`]
    fn serialize_to_bytes(
        &self,
        encoding: Encodings,
    ) -> Result<Vec<u8>, EncodingError>;
    /// Turn bytes into self
    ///
    /// # Errors
    /// [`EncodingError`]
    fn deserialize_to_self(
        bytes: &[u8],
        encoding: Encodings,
    ) -> Result<Self, EncodingError>;
}
