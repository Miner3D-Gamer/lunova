#![allow(clippy::too_long_first_doc_paragraph)]
use crate::users::user_id::UserID;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
/// Where a message is (going)
pub enum ChatLocation {
    /// To a person private chat
    DM(UserID),
    /// To a group chat
    Group(String),
    /// To a guild
    Guild,
}

/// Messages the only client should send the server
pub mod client;
/// Messages both the client and server can send each other
pub mod mutual;
/// Messages only the server should send the client
pub mod server;

/// Reporting reasons
pub mod reports;
