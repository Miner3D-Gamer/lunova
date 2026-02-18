use super::ChatLocation;
use crate::{shared::Timestamp, users::user_id::UserID};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
/// An api call both the client and server could request from one another
pub enum MutualCall {
    /// Send a new message
    SendNewMessage(SendNewMessage),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
/// A message from the client to the server mentioning the creation of a new message
pub struct SendNewMessage {
    /// Message version
    pub version: usize,
    /// Sender ID
    pub sender: UserID,
    /// At what time the message was send
    pub timestamp: Timestamp,
    /// The content of the message
    pub content: String,
    /// Where the message is going
    pub destination: ChatLocation,
}
