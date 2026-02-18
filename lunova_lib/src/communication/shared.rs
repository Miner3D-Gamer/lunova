use crate::{shared::Timestamp, users::user_id::UserID};

#[derive(Debug, Clone, PartialEq, Eq)]
/// A text based channel
pub struct TextChat {
    /// All messages
    pub messages: indexmap::IndexMap<MessageId, Message>,
    /// The title
    pub name: String,
    /// A description
    pub description: String,
    /// Any pinned messages
    pub pinned: Vec<String>,
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
/// A singular text message
pub struct Message {
    /// Who send the message
    pub sender: UserID,
    /// When the message/edit was send (Client side)
    pub timestamp_send: Timestamp,
    /// When the message arrived (Server side)
    pub timestamp: Timestamp,
    /// When the message was edited (Server side)
    pub edited_timestamp: Timestamp,
    /// What text the message contains
    pub content: String,
    /// Any reactions to the message
    pub reactions: Vec<(Vec<UserID>, String)>,
}
/// A message id
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MessageId {
    /// A unique id for a message inside of a singular chat
    pub id: u64,
}
impl MessageId {
    #[must_use]
    /// Get the underlying ID
    pub const fn get_id(&self) -> u64 {
        self.id
    }
}
