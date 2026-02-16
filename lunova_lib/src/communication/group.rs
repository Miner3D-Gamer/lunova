use crate::{communication::shared::TextChat, users::user_id::UserID};
#[derive(Debug, Clone, PartialEq, Eq)]
/// A group is a dm with multiple people who have the same rights
pub struct Group {
    /// All people in the chat
    pub participants: Vec<UserID>,
    /// The actual chat
    pub chat: TextChat,
}
