#![allow(missing_docs)]
use mirl::impl_from_patch_self;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
/// A singular text message
pub struct MessageVersion0 {
    pub timestamp_send: std::time::SystemTime,
    pub sender: UserIDVersion0,
    pub timestamp: std::time::SystemTime,
    pub edited_timestamp: std::time::SystemTime,
    pub content: String,
    pub reactions: Vec<(Vec<UserIDVersion0>, String)>,
}
#[repr(C)]
/// A message id
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MessageIdVersion0 {
    pub id: u64,
}
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
/// A text based channel
pub struct TextChatVersion0 {
    /// All messages
    pub messages: Vec<(MessageIdVersion0, MessageVersion0)>,
    /// The title
    pub name: String,
    /// A description
    pub description: String,
    /// Any pinned messages
    pub pinned: Vec<String>,
}
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
/// An "ID", an identifier that is not allowed to be same for two users
pub struct UserIDVersion0 {
    pub id: String,
}
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// A unique id for dms
pub struct DMIDVersion0 {
    pub id: u64,
}

#[repr(C)]
/// The DMs saved on the server
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AllDMsVersion0 {
    /// The account that was created earlier is the first, then comes the other
    pub dm_participants: Vec<(UserIDVersion0, UserIDVersion0, DMIDVersion0)>,
    /// All dms
    pub dms: std::collections::HashMap<DMIDVersion0, TextChatVersion0>,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
/// A list of all users
pub struct UsersVersion0 {
    /// All registered accounts
    pub users: Vec<UserVersion0>,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
/// The first server state version only saving dms and users
pub struct ServerStateVersion0 {
    /// Peer to peer messaging
    pub dms: AllDMsVersion0,
    /// All users
    pub users: UsersVersion0,
    /// All pending friend requests
    pub pending_friend_requests: PendingFriendRequestsVersion0,
    /// All server related configs
    pub configs: ServerConfigsVersion0,
}
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
/// A plain old user
pub struct UserVersion0 {
    /// A unique ID no other user is allowed to have
    pub id: UserIDVersion0,
    /// The display username
    pub username: String,
    /// What friends a user has
    pub friends: Vec<UserIDVersion0>,
    /// Reports that
    pub reports_against_this_user: Vec<ReportVersion0>,
    /// The last interaction the user had with the server
    pub last_online: std::time::SystemTime,
    /// When the account was created
    pub account_creation_date: std::time::SystemTime,
    /// A status a user may set for a specified amount of time
    pub status: StatusVersion0,
    /// A self assigned user description
    pub bio: String,
    /// How the user is doing from our pov
    pub standing: StandingVersion0,
}
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
/// How the user is doing from our pov
pub struct StandingVersion0 {
    /// How many times a user may have been warned before
    pub times_warned_reason: Vec<ReportVersion0>,
    // /// How many times a user has been restricted before
    // times_restricted: u8,
    /// How many times a user has been blocked before
    pub times_blocked_reason: Vec<ReportVersion0>,
    /// The current standing
    pub current_standing: StandingTypeVersion0,
    /// How suspicious the user is
    pub watchfulness: u8,
    /// The maximum watchfulness the user ever has at the same time
    pub highest_watchfulness: u8,
    /// The total watchfulness of the user
    pub total_watchfulness: u8,
}
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Copy)]
/// The rights of the user
pub enum StandingTypeVersion0 {
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
#[repr(C)]
/// Control over the standing a user may has
pub struct StandingConfigVersion0 {
    /// How many days the user
    pub reduce_watchfulness_every: u16,
    /// If a user should automatically banned upon reaching a certain total watchfulness
    pub automatic_ban_at_total_watchfulness: Option<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[repr(C)]
/// A often temporary "mini" bio
pub struct StatusVersion0 {
    pub expiration_date: std::time::SystemTime,
    pub message: String,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(C)]
/// Friend requests that are out and ingoing
pub struct PendingFriendRequestsVersion0 {
    /// Ones that still have to be accepted/canceled
    pub pending:
        Vec<(UserIDVersion0, Vec<(UserIDVersion0, std::time::SystemTime)>)>,
}
#[derive(Debug, Clone, PartialEq, Eq, Copy)]
#[repr(C)]
/// A collection of all server related configs
pub struct ServerConfigsVersion0 {
    /// [`StandingConfigVersion0`]
    pub user_standing: StandingConfigVersion0,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(C)]
/// A report
pub struct ReportVersion0 {
    /// What the report is about
    pub reason: ReportReasonsVersion0,
    /// Who the report is about
    pub target: UserIDVersion0,
    /// If the report created in the context of a message rather over the profile itself
    pub message: Option<MessageIdVersion0>,
    /// Additional notes that may aid in taking action against the target
    pub reporter_notes: String,
    /// The one who created the report. Is used for giving the information that the given report went through and the reported person was banned.
    pub from: UserIDVersion0,
}
#[repr(C)]
/// Every report must be humanly reviewed.
///
/// Ai and other automated "moderation systems" are too prone to give false results, especially with sensitive topics
///
/// Remember to stay within the law
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Copy)]
pub enum ReportSeverityVersion0 {
    /// Things we truly don't care about like the "child proofing" messages
    Trivial,

    /// # Report reason we don't care about.
    /// Most often solvable by just blocking someone or not being a bitch
    ///
    /// It is the responsibility of the server admin to keep their chats moderated, not ours
    ///
    /// ---
    ///
    /// These may get redirected to the server owner if one exists or a prompt to mute/block them shall be displayed
    SelfOrServerModeration,
    /// Reports we as the server owner do care about.
    HumanReview,
}
#[repr(C)]
/// How severe a report actually is
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Copy)]
pub struct ReportReasonSeverityVersion0 {
    /// [`ReportReasonsVersion0::IDoNotLikeThis`]
    pub i_do_not_like_this: ReportSeverityVersion0,
    /// [`ReportReasonsVersion0::TheyAreAnnoying`]
    pub they_are_annoying: ReportSeverityVersion0,
    /// [`ReportReasonsVersion0::HarassmentOrBullying`]
    pub harassment_or_bullying: ReportSeverityVersion0,
    /// [`ReportReasonsVersion0::Spam`]
    pub spam: ReportSeverityVersion0,
    /// [`ReportReasonsVersion0::ScamFraud`]
    pub scam_fraud: ReportSeverityVersion0,
    /// [`ReportReasonsVersion0::MalwareDistribution`]
    pub malware_distribution: ReportSeverityVersion0,
    /// [`ReportReasonsVersion0::ThreatsOrViolence`]
    pub threats_or_violence: ReportSeverityVersion0,
    /// [`ReportReasonsVersion0::DangerousMisinformation`]
    pub dangerous_misinformation: ReportSeverityVersion0,
    /// [`ReportReasonsVersion0::SelfHarm`]
    pub self_harm: ReportSeverityVersion0,
    /// [`ReportReasonsVersion0::Impersonation`]
    pub impersonation: ReportSeverityVersion0,
    /// [`ReportReasonsVersion0::RevealingPrivateInformation`]
    pub revealing_private_information: ReportSeverityVersion0,
    /// [`ReportReasonsVersion0::DistributionOfAccounts`]
    pub distribution_of_accounts: ReportSeverityVersion0,
    /// [`ReportReasonsVersion0::StolenAccount`]
    pub stolen_account: ReportSeverityVersion0,
    /// [`ReportReasonsVersion0::SexualContent`]
    pub sexual_content: ReportSeverityVersion0,
    /// [`ReportReasonsVersion0::BanEvasion`]
    pub ban_evasion: ReportSeverityVersion0,
    /// [`ReportReasonsVersion0::Other`]
    pub other: ReportSeverityVersion0,
    /// [`ReportReasonsVersion0::CopyrightOrIpInfringement`]
    pub copyright_or_ip_infringement: ReportSeverityVersion0,
}
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
/// Any reason to report a chat message or person
pub enum ReportReasonsVersion0 {
    /// When you don't like someone
    IDoNotLikeThis,
    /// When someone is very annoying
    TheyAreAnnoying,
    /// Harassment/Online bullying/Hate speech/encouraging self harm/grooming/coordinated harassment (Just block the damn people!)
    ///
    /// All messages in these categories are subjective and can most likely not be definitively be proven (/can easily be taken out of context)
    HarassmentOrBullying,
    /// When someone does nothing but fill the chat with spam
    Spam,
    /// Anything from sending malicious website links to phishing, not to be confused with [`StolenAccount`](Self::StolenAccount)
    ScamFraud,
    /// While [`ScamFraud`](Self::ScamFraud) goes hand in hand with this, if the intent is to distribute malware (files which need to be executed) it goes in this category
    MalwareDistribution,
    /// We cannot determine the context meaning we are unreliable.
    ///
    /// An extra prompt to alert local law enforcement may be displayed
    ThreatsOrViolence,
    /// From stupid challenges to false safety information
    DangerousMisinformation,
    /// From self harm to suicide, professional help may be needed which the person is apparently not getting
    SelfHarm,
    /// When acting out being another person
    ///
    /// Everyone should keep in their own lane
    Impersonation,
    /// When a message reveals private information about someone
    ///
    /// Everyone should be kept anonymous
    RevealingPrivateInformation,
    /// One may not sell nor buy an account
    ///
    /// Sharing an account is allowed but as soon as money is involved or it turns into [`StolenAccount`](Self::StolenAccount)
    DistributionOfAccounts,
    /// When someone steals the account of someone else, different from [Impersonation](Self::Impersonation) as the account must have previously been owned by someone else
    StolenAccount,
    /// Distributing illegal substances includes drugs, alcohol, and more
    DistributingIllegalSubstances,
    /// While sexual content is allowed, this category covers all that is non consensual sexual acts to sexual abuse to child pornography
    SexualContent,
    /// When someones creates a new account to get around their old one being banned/withheld
    ///
    /// This includes alt accounts when the main account has been compromised
    BanEvasion,
    /// When the problem isn't stated anywhere, we don't know how severe it is so it is put at the highest level
    Other(String),
    /// Not to be confused with [`RevealingPrivateInformation`](Self::RevealingPrivateInformation), when content that is restricted to certain people is shared
    ///
    /// Redacted content also fits into this category
    CopyrightOrIpInfringement,
}
impl_from_patch_self!(
    UserVersion0,
    UsersVersion0,
    ReportVersion0,
    StatusVersion0,
    MessageVersion0,
    StandingVersion0,
    TextChatVersion0,
    ServerStateVersion0,
    StandingTypeVersion0,
    ReportReasonsVersion0,
    ServerConfigsVersion0,
    StandingConfigVersion0,
    DMIDVersion0,
);
