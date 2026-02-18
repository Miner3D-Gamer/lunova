use crate::users::user::Accounts;

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
impl AllAccounts {
    #[must_use]
    /// Check if an account exists
    pub fn contains_account(&self, account: &Accounts) -> bool {
        self.contains_account_id(&account.id)
    }
    #[must_use]
    /// Check if an account id exists
    pub fn contains_account_id(&self, id: &user_id::UserID) -> bool {
        self.users.iter().any(|x| x.id.eq(id))
    }
}
