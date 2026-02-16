use crate::communication::shared::TextChat;
/// Permission related stuff
pub mod permissions;
use permissions::*;

#[derive(Debug, Clone, PartialEq, Eq)]
/// A collection of communication channels with customizable permissions
pub struct Guild {
    /// The guild root
    pub category: GuildCategory,
}
#[derive(Debug, Clone, PartialEq, Eq)]
/// A category holding sub channels as well as permissions
pub struct GuildCategory {
    chat_permissions: Permissions<RawChatPermissions>,
    channel_permissions: Permissions<RawChannelPermissions>,
    channels: indexmap::IndexMap<usize, GuildChannelType>,
}
#[derive(Debug, Clone, PartialEq, Eq)]
/// An object, either a file (channel) or sub folder (category)
pub enum GuildChannelType {
    /// A sub directory
    Category(GuildCategory),
    /// A channel with the primary communication type of text
    TextChat(TextChannel),
    /// A channel with the primary communication type of voice
    ///
    /// # UNSUPPORTED
    VoiceChat,
    /// A ticket like chat type where one is able to create individual posts
    ///
    /// # UNSUPPORTED
    Forum,
}
#[derive(Debug, Clone, PartialEq, Eq)]
/// A text channel ¯\_(ツ)_/¯
pub struct TextChannel {
    /// The local permissions
    pub chat_permissions: Permissions<RawChatPermissions>,
    /// The chat itself
    pub chat: TextChat,
}
