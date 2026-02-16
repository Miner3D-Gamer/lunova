//! Reminder:
//! We as the server owners are not responsible for the content distributed on our platform.
//! We shall not determine the limitation/restrictions of content based on morals but instead based on factual laws: if your morality says "I wanna report" but the law says "We don't care about that" then we won't care either.
//! If someone is to make people uncomfortable, it is the responsibility of those people/higher ranking individuals to cut this person out. Again, not us.
//!
//! We do not look at or share any of the data stored with the exception of reported content.
//! When someone is reported, their current bio, status, name etc. will be saved and reviewed. If that report also included a message as context, the given chat will be fully logged and all attachments locally saved for reviewing.

use crate::{communication::shared::MessageId, users::user_id::UserID};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
/// A report
pub struct Report {
    /// What the report is about
    pub reason: ReportReasons,
    /// Who the report is about
    pub target: UserID,
    /// If the report created in the context of a message rather over the profile itself
    pub message: Option<MessageId>,
    /// Additional notes that may aid in taking action against the target
    pub reporter_notes: String,
    /// The one who created the report. Is used for giving the information that the given report went through and the reported person was banned.
    pub from: UserID,
}

/// Every report must be humanly reviewed.
///
/// Ai and other automated "moderation systems" are too prone to give false results, especially with sensitive topics
///
/// Remember to stay within the law
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Copy)]
pub enum ReportSeverity {
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
/// How severe a report actually is
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Copy)]
pub struct ReportReasonSeverity {
    /// [`ReportReasons::IDoNotLikeThis`]
    pub i_do_not_like_this: ReportSeverity,
    /// [`ReportReasons::TheyAreAnnoying`]
    pub they_are_annoying: ReportSeverity,
    /// [`ReportReasons::HarassmentOrBullying`]
    pub harassment_or_bullying: ReportSeverity,
    /// [`ReportReasons::Spam`]
    pub spam: ReportSeverity,
    /// [`ReportReasons::ScamFraud`]
    pub scam_fraud: ReportSeverity,
    /// [`ReportReasons::MalwareDistribution`]
    pub malware_distribution: ReportSeverity,
    /// [`ReportReasons::ThreatsOrViolence`]
    pub threats_or_violence: ReportSeverity,
    /// [`ReportReasons::DangerousMisinformation`]
    pub dangerous_misinformation: ReportSeverity,
    /// [`ReportReasons::SelfHarm`]
    pub self_harm: ReportSeverity,
    /// [`ReportReasons::Impersonation`]
    pub impersonation: ReportSeverity,
    /// [`ReportReasons::RevealingPrivateInformation`]
    pub revealing_private_information: ReportSeverity,
    /// [`ReportReasons::DistributionOfAccounts`]
    pub distribution_of_accounts: ReportSeverity,
    /// [`ReportReasons::StolenAccount`]
    pub stolen_account: ReportSeverity,
    /// [`ReportReasons::SexualContent`]
    pub sexual_content: ReportSeverity,
    /// [`ReportReasons::BanEvasion`]
    pub ban_evasion: ReportSeverity,
    /// [`ReportReasons::Other`]
    pub other: ReportSeverity,
    /// [`ReportReasons::CopyrightOrIpInfringement`]
    pub copyright_or_ip_infringement: ReportSeverity,
}
impl Default for ReportReasonSeverity {
    fn default() -> Self {
        Self {
            i_do_not_like_this: ReportSeverity::Trivial,
            they_are_annoying: ReportSeverity::Trivial,
            harassment_or_bullying: ReportSeverity::SelfOrServerModeration,
            spam: ReportSeverity::SelfOrServerModeration,
            scam_fraud: ReportSeverity::SelfOrServerModeration,
            malware_distribution: ReportSeverity::SelfOrServerModeration,
            threats_or_violence: ReportSeverity::SelfOrServerModeration,
            dangerous_misinformation: ReportSeverity::SelfOrServerModeration,
            self_harm: ReportSeverity::HumanReview,
            impersonation: ReportSeverity::HumanReview,
            revealing_private_information: ReportSeverity::HumanReview,
            distribution_of_accounts: ReportSeverity::HumanReview,
            stolen_account: ReportSeverity::HumanReview,
            sexual_content: ReportSeverity::HumanReview,
            ban_evasion: ReportSeverity::HumanReview,
            other: ReportSeverity::HumanReview,
            copyright_or_ip_infringement: ReportSeverity::HumanReview,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
/// Any reason to report a chat message or person
pub enum ReportReasons {
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
