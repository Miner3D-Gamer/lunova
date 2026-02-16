use crate::{communication::shared::TextChat, users::user_id::UserID};

// #[derive(Debug, Clone, PartialEq, Eq)]
// /// A chat between 2 people
// pub struct DM {
//     // /// The person with the account that was created first
//     // pub participant_1: UserID,
//     /// The person with the account that was created second
//     pub participant: UserID,
//     /// The chat between these people
//     pub chat: TextChat,
// }
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// A unique id for dms
pub struct DMID {
    /// The internally used id
    pub id: u64,
}

/// The DMs saved on the server
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct AllDMs {
    /// The account that was created earlier is the first, then comes the other
    pub dm_participants: Vec<(UserID, UserID, DMID)>,
    /// All dms
    pub dms: std::collections::HashMap<DMID, TextChat>,
}
