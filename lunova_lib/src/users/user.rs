use crate::{
    shared::Timestamp, throughput::reports::Report, users::user_id::UserID,
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
/// A plain old user
pub struct Accounts {
    /// A unique ID no other user is allowed to have
    pub id: UserID,
    /// The display username
    pub username: String,
    /// What friends a user has
    pub friends: Vec<UserID>,
    /// Reports that
    pub reports_against_this_user: Vec<Report>,
    /// The last interaction the user had with the server
    pub last_online: Timestamp,
    /// When the account was created
    pub account_creation_date: Timestamp,
    /// A status a user may set for a specified amount of time
    pub status: Option<Status>,
    /// A self assigned user description
    pub bio: String,
    /// How the user is doing from our pov
    pub standing: Standing,
}
impl Accounts {
    #[must_use] 
    /// Create a blank new user account
    pub fn new(id: UserID, username: String) -> Self {
        Self {
            id,
            username,
            friends: Vec::new(),
            reports_against_this_user: Vec::new(),
            last_online: Timestamp::now(),
            account_creation_date: Timestamp::now(),
            status: None,
            bio: String::new(),
            standing: Standing::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
/// A often temporary "mini" bio
pub struct Status {
    /// When the status should expire
    pub expiration_date: Timestamp,
    /// The the current status says
    pub message: String,
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
/// How the user is doing from our pov
pub struct Standing {
    /// How many times a user may have been warned before and for what reason
    pub times_warned_reason: Vec<Report>,
    // /// How many times a user has been restricted before
    // times_restricted: u8,
    /// How many times a user has been blocked before
    pub times_blocked_reason: Vec<Report>,
    /// The current standing
    pub current_standing: StandingType,
    /// How suspicious the user is
    pub watchfulness: u8,
    /// The maximum watchfulness the user ever has at the same time
    pub highest_watchfulness: u8,
    /// The total watchfulness of the user
    pub total_watchfulness: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Copy, Default)]
/// The rights of the user
pub enum StandingType {
    #[default]
    /// Everything about the user is normal
    AllGood,
    /// If the user has been in some controversy
    HasBeenWarned,
    // /// The user is not allowed to send any messages, friend requests, or create chats
    // RestrictedToViewOnly,
    /// The user is not allowed to do anything
    Blocked,
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Copy)]
/// Control over the standing a user may has
pub struct StandingConfig {
    /// How many days the user
    pub reduce_watchfulness_every: u16,
    /// If a user should automatically banned upon reaching a certain total watchfulness
    pub automatic_ban_at_total_watchfulness: Option<u8>,
}

impl Default for StandingConfig {
    fn default() -> Self {
        Self {
            reduce_watchfulness_every: 30,
            automatic_ban_at_total_watchfulness: Some(u8::MAX),
        }
    }
}
