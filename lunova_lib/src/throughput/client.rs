use crate::{communication::shared::MessageId, throughput::ChatLocation};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
/// Api calls only the client should send
pub enum ClientCall {
    /// [`GetChatContents`]
    GetChatContents(GetChatContents),
}
/// Get X chat messages, if offset is None it is assumed that the latest messages are to be loaded
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct GetChatContents {
    location: ChatLocation,
    how_many: u16,
    offset: Option<MessageId>,
}
