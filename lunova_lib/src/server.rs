use crate::{
    communication::dm::AllDMs,
    shared::Timestamp,
    users::{
        AllAccounts,
        user::StandingConfig,
        user_id::{IDConstraints, UserID},
    },
};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
/// The state of the server
pub struct ServerState {
    /// All dms
    pub dms: AllDMs,
    /// All users
    pub users: AllAccounts,
    /// All pending friend requests
    pub pending_friend_requests: PendingFriendRequests,
    /// All server related configs
    pub configs: ServerConfigs,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
/// Friend requests that are out and ingoing
pub struct PendingFriendRequests {
    /// Ones that still have to be accepted/canceled
    pub pending: Vec<(UserID, Vec<(UserID, Timestamp)>)>,
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
/// A collection of all server related configs
pub struct ServerConfigs {
    /// [`StandingConfig`]
    pub user_standing: StandingConfig,
    /// [`IDConstraints`]
    pub user_id_constraints: IDConstraints,
}

// /// Load the server state from disk
// pub trait LoadServerState {
//     /// Compatibility for forward+backward compatibility
//     const VERSION: u16;
// }
// #[derive(Debug, Clone, PartialEq, Eq)]
// pub struct SaveServerState {}
// impl SaveServerState {
//     /// The current version for saving in hopes of forward compatibility
//     pub const VERSION: u16 = 0;
// }
