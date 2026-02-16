use mirl::prelude::{FromPatch, IntoPatch};

use crate::{
    communication::{
        dm::{AllDMs, DMID},
        shared::{Message, MessageId, TextChat},
    },
    fs::server::versions::v0::{
        AllDMsVersion0, DMIDVersion0, MessageIdVersion0, MessageVersion0,
        PendingFriendRequestsVersion0, ReportReasonsVersion0, ReportVersion0,
        ServerConfigsVersion0, ServerStateVersion0, StandingConfigVersion0,
        StandingTypeVersion0, StandingVersion0, StatusVersion0,
        TextChatVersion0, UserIDVersion0, UserVersion0, UsersVersion0,
    },
    server::{PendingFriendRequests, ServerConfigs, ServerState},
    throughput::reports::{Report, ReportReasons},
    users::{
        AllAccounts,
        user::{Standing, StandingConfig, StandingType, Status, Accounts},
        user_id::UserID,
    },
};

impl FromPatch<ServerStateVersion0> for ServerState {
    fn from_value(value: ServerStateVersion0) -> Self {
        Self {
            dms: value.dms.into_value(),
            users: value.users.into_value(),
            configs: value.configs.into_value(),
            pending_friend_requests: value.pending_friend_requests.into_value(),
        }
    }
}
impl FromPatch<ServerState> for ServerStateVersion0 {
    fn from_value(value: ServerState) -> Self {
        Self {
            dms: value.dms.into_value(),
            users: value.users.into_value(),
            configs: value.configs.into_value(),
            pending_friend_requests: value.pending_friend_requests.into_value(),
        }
    }
}

// impl FromPatch<DMVersion0> for DM {
//     fn from_value(value: DMVersion0) -> Self {
//         Self {
//             participant_1: value.participant_1.into_value(),
//             participant_2: value.participant_2.into_value(),
//             chat: value.chat.into_value(),
//         }
//     }
// }
// impl FromPatch<DM> for DMVersion0 {
//     fn from_value(value: DM) -> Self {
//         Self {
//             participant_1: value.participant_1.into_value(),
//             participant_2: value.participant_2.into_value(),
//             chat: value.chat.into_value(),
//         }
//     }
// }

impl FromPatch<UserIDVersion0> for UserID {
    fn from_value(value: UserIDVersion0) -> Self {
        Self {
            id: value.id,
        }
    }
}

impl FromPatch<UserID> for UserIDVersion0 {
    fn from_value(value: UserID) -> Self {
        Self {
            id: value.id,
        }
    }
}

impl FromPatch<TextChatVersion0> for TextChat {
    fn from_value(value: TextChatVersion0) -> Self {
        Self {
            messages: value.messages.into_value(),
            name: value.name,
            description: value.description,
            pinned: value.pinned,
        }
    }
}
impl FromPatch<TextChat> for TextChatVersion0 {
    fn from_value(value: TextChat) -> Self {
        Self {
            messages: value.messages.into_value(),
            name: value.name,
            description: value.description,
            pinned: value.pinned,
        }
    }
}
impl FromPatch<MessageIdVersion0> for MessageId {
    fn from_value(value: MessageIdVersion0) -> Self {
        Self {
            id: value.id,
        }
    }
}

impl FromPatch<MessageId> for MessageIdVersion0 {
    fn from_value(value: MessageId) -> Self {
        Self {
            id: value.id,
        }
    }
}

impl FromPatch<Message> for MessageVersion0 {
    fn from_value(value: Message) -> Self {
        Self {
            timestamp_send: value.timestamp_send,
            sender: value.sender.into_value(),
            timestamp: value.timestamp,
            edited_timestamp: value.edited_timestamp,
            content: value.content,
            reactions: value.reactions.into_value(),
        }
    }
}
impl FromPatch<MessageVersion0> for Message {
    fn from_value(value: MessageVersion0) -> Self {
        Self {
            timestamp_send: value.timestamp_send,
            sender: value.sender.into_value(),
            timestamp: value.timestamp,
            edited_timestamp: value.edited_timestamp,
            content: value.content,
            reactions: value.reactions.into_value(),
        }
    }
}
impl FromPatch<AllDMsVersion0> for AllDMs {
    fn from_value(value: AllDMsVersion0) -> Self {
        Self {
            dm_participants: value.dm_participants.into_value(),
            dms: value.dms.into_value(),
        }
    }
}

impl FromPatch<AllDMs> for AllDMsVersion0 {
    fn from_value(value: AllDMs) -> Self {
        Self {
            dm_participants: value.dm_participants.into_value(),
            dms: value.dms.into_value(),
        }
    }
}

impl FromPatch<DMIDVersion0> for DMID {
    fn from_value(value: DMIDVersion0) -> Self {
        Self {
            id: value.id,
        }
    }
}

impl FromPatch<DMID> for DMIDVersion0 {
    fn from_value(value: DMID) -> Self {
        Self {
            id: value.id,
        }
    }
}

impl FromPatch<PendingFriendRequestsVersion0> for PendingFriendRequests {
    fn from_value(value: PendingFriendRequestsVersion0) -> Self {
        Self {
            pending: value.pending.into_value(),
        }
    }
}
impl FromPatch<PendingFriendRequests> for PendingFriendRequestsVersion0 {
    fn from_value(value: PendingFriendRequests) -> Self {
        Self {
            pending: value.pending.into_value(),
        }
    }
}

impl FromPatch<ServerConfigsVersion0> for ServerConfigs {
    fn from_value(value: ServerConfigsVersion0) -> Self {
        Self {
            user_standing: value.user_standing.into_value(),
        }
    }
}

impl FromPatch<ServerConfigs> for ServerConfigsVersion0 {
    fn from_value(value: ServerConfigs) -> Self {
        Self {
            user_standing: value.user_standing.into_value(),
        }
    }
}

impl FromPatch<StandingConfigVersion0> for StandingConfig {
    fn from_value(value: StandingConfigVersion0) -> Self {
        Self {
            reduce_watchfulness_every: value.reduce_watchfulness_every,
            automatic_ban_at_total_watchfulness: value
                .automatic_ban_at_total_watchfulness,
        }
    }
}

impl FromPatch<StandingConfig> for StandingConfigVersion0 {
    fn from_value(value: StandingConfig) -> Self {
        Self {
            reduce_watchfulness_every: value.reduce_watchfulness_every,
            automatic_ban_at_total_watchfulness: value
                .automatic_ban_at_total_watchfulness,
        }
    }
}

impl FromPatch<UsersVersion0> for AllAccounts {
    fn from_value(value: UsersVersion0) -> Self {
        Self {
            users: value.users.into_value(),
        }
    }
}
impl FromPatch<AllAccounts> for UsersVersion0 {
    fn from_value(value: AllAccounts) -> Self {
        Self {
            users: value.users.into_value(),
        }
    }
}

impl FromPatch<UserVersion0> for Accounts {
    fn from_value(value: UserVersion0) -> Self {
        Self {
            id: value.id.into_value(),
            username: value.username,
            friends: value.friends.into_value(),
            reports_against_this_user: value
                .reports_against_this_user
                .into_value(),
            last_online: value.last_online,
            account_creation_date: value.account_creation_date,
            status: value.status.into_value(),
            bio: value.bio,
            standing: value.standing.into_value(),
        }
    }
}

impl FromPatch<Accounts> for UserVersion0 {
    fn from_value(value: Accounts) -> Self {
        Self {
            id: value.id.into_value(),
            username: value.username,
            friends: value.friends.into_value(),
            reports_against_this_user: value
                .reports_against_this_user
                .into_value(),
            last_online: value.last_online,
            account_creation_date: value.account_creation_date,
            status: value.status.into_value(),
            bio: value.bio,
            standing: value.standing.into_value(),
        }
    }
}

impl FromPatch<ReportVersion0> for Report {
    fn from_value(value: ReportVersion0) -> Self {
        Self {
            reason: value.reason.into_value(),
            target: value.target.into_value(),
            message: value.message.into_value(),
            reporter_notes: value.reporter_notes,
            from: value.from.into_value(),
        }
    }
}

impl FromPatch<Report> for ReportVersion0 {
    fn from_value(value: Report) -> Self {
        Self {
            reason: value.reason.into_value(),
            target: value.target.into_value(),
            message: value.message.into_value(),
            reporter_notes: value.reporter_notes,
            from: value.from.into_value(),
        }
    }
}

impl FromPatch<Status> for StatusVersion0 {
    fn from_value(value: Status) -> Self {
        Self {
            expiration_date: value.expiration_date,
            message: value.message,
        }
    }
}

impl FromPatch<StatusVersion0> for Status {
    fn from_value(value: StatusVersion0) -> Self {
        Self {
            expiration_date: value.expiration_date,
            message: value.message,
        }
    }
}

impl FromPatch<StandingVersion0> for Standing {
    fn from_value(value: StandingVersion0) -> Self {
        Self {
            times_warned_reason: value.times_warned_reason.into_value(),
            times_blocked_reason: value.times_blocked_reason.into_value(),
            current_standing: value.current_standing.into_value(),
            watchfulness: value.watchfulness,
            highest_watchfulness: value.highest_watchfulness,
            total_watchfulness: value.total_watchfulness,
        }
    }
}

impl FromPatch<Standing> for StandingVersion0 {
    fn from_value(value: Standing) -> Self {
        Self {
            times_warned_reason: value.times_warned_reason.into_value(),
            times_blocked_reason: value.times_blocked_reason.into_value(),
            current_standing: value.current_standing.into_value(),
            watchfulness: value.watchfulness,
            highest_watchfulness: value.highest_watchfulness,
            total_watchfulness: value.total_watchfulness,
        }
    }
}

impl FromPatch<StandingTypeVersion0> for StandingType {
    fn from_value(value: StandingTypeVersion0) -> Self {
        match value {
            StandingTypeVersion0::AllGood => Self::AllGood,
            StandingTypeVersion0::Blocked => Self::Blocked,
            StandingTypeVersion0::HasBeenWarned => Self::HasBeenWarned,
        }
    }
}

impl FromPatch<StandingType> for StandingTypeVersion0 {
    fn from_value(value: StandingType) -> Self {
        match value {
            StandingType::AllGood => Self::AllGood,
            StandingType::Blocked => Self::Blocked,
            StandingType::HasBeenWarned => Self::HasBeenWarned,
        }
    }
}

impl FromPatch<ReportReasonsVersion0> for ReportReasons {
    fn from_value(value: ReportReasonsVersion0) -> Self {
        match value {
            ReportReasonsVersion0::IDoNotLikeThis => Self::IDoNotLikeThis,
            ReportReasonsVersion0::TheyAreAnnoying => Self::TheyAreAnnoying,
            ReportReasonsVersion0::HarassmentOrBullying => {
                Self::HarassmentOrBullying
            }
            ReportReasonsVersion0::Spam => Self::Spam,
            ReportReasonsVersion0::ScamFraud => Self::ScamFraud,
            ReportReasonsVersion0::MalwareDistribution => {
                Self::MalwareDistribution
            }
            ReportReasonsVersion0::ThreatsOrViolence => Self::ThreatsOrViolence,
            ReportReasonsVersion0::DangerousMisinformation => {
                Self::DangerousMisinformation
            }
            ReportReasonsVersion0::SelfHarm => Self::SelfHarm,
            ReportReasonsVersion0::Impersonation => Self::Impersonation,
            ReportReasonsVersion0::RevealingPrivateInformation => {
                Self::RevealingPrivateInformation
            }
            ReportReasonsVersion0::DistributionOfAccounts => {
                Self::DistributionOfAccounts
            }
            ReportReasonsVersion0::StolenAccount => Self::StolenAccount,
            ReportReasonsVersion0::DistributingIllegalSubstances => {
                Self::DistributingIllegalSubstances
            }
            ReportReasonsVersion0::SexualContent => Self::SexualContent,
            ReportReasonsVersion0::BanEvasion => Self::BanEvasion,
            ReportReasonsVersion0::Other(s) => Self::Other(s),
            ReportReasonsVersion0::CopyrightOrIpInfringement => {
                Self::CopyrightOrIpInfringement
            }
        }
    }
}
impl FromPatch<ReportReasons> for ReportReasonsVersion0 {
    fn from_value(value: ReportReasons) -> Self {
        match value {
            ReportReasons::IDoNotLikeThis => Self::IDoNotLikeThis,
            ReportReasons::TheyAreAnnoying => Self::TheyAreAnnoying,
            ReportReasons::HarassmentOrBullying => Self::HarassmentOrBullying,
            ReportReasons::Spam => Self::Spam,
            ReportReasons::ScamFraud => Self::ScamFraud,
            ReportReasons::MalwareDistribution => Self::MalwareDistribution,
            ReportReasons::ThreatsOrViolence => Self::ThreatsOrViolence,
            ReportReasons::DangerousMisinformation => {
                Self::DangerousMisinformation
            }
            ReportReasons::SelfHarm => Self::SelfHarm,
            ReportReasons::Impersonation => Self::Impersonation,
            ReportReasons::RevealingPrivateInformation => {
                Self::RevealingPrivateInformation
            }
            ReportReasons::DistributionOfAccounts => {
                Self::DistributionOfAccounts
            }
            ReportReasons::StolenAccount => Self::StolenAccount,
            ReportReasons::DistributingIllegalSubstances => {
                Self::DistributingIllegalSubstances
            }
            ReportReasons::SexualContent => Self::SexualContent,
            ReportReasons::BanEvasion => Self::BanEvasion,
            ReportReasons::Other(s) => Self::Other(s),
            ReportReasons::CopyrightOrIpInfringement => {
                Self::CopyrightOrIpInfringement
            }
        }
    }
}

// impl FromPatch<ReportReasonsVersion0> for ReportReasons {
//     fn from_value(value: Standing) -> Self {
//         Self {
//             times_warned_reason: value.times_warned_reason.into_value(),
//             times_blocked_reason: value.times_blocked_reason.into_value(),
//             current_standing: value.current_standing.into_value(),
//             watchfulness: value.watchfulness,
//             highest_watchfulness: value.highest_watchfulness,
//             total_watchfulness: value.total_watchfulness,
//         }
//     }
// }
