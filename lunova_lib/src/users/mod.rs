/// A user itself
pub mod user;
/// Stuff that has to do with the unique identifiers of a user
pub mod user_id;
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
/// A list of all users
pub struct AllAccounts {
    /// All registered accounts
    pub users: Vec<user::Accounts>,
}
