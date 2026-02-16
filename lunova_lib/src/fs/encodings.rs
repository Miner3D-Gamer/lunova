#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
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
