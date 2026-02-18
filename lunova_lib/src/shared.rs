use mirl::misc::EasyUnwrapUnchecked;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// A timestamp
pub struct Timestamp {
    /// How many seconds the event happened after epoch
    pub after_epoch: u128,
}
impl Timestamp {
    #[must_use]
    /// Create a new timestamp for now
    pub fn now() -> Self {
        Self {
            after_epoch: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .easy_unwrap_unchecked()
                .as_millis(),
        }
    }
}
