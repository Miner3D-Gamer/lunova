use mirl::prelude::IntoPatch;

use crate::users::user_id::UserID;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
/// A simple true/false is not enough for most decisions in the permissions tab
///
/// Using [`InheritFromParent`](Self::InheritFromParent) on a root causes it to use [`Default`](Self::Default) instead
pub enum PermissionDecision {
    /// True
    Yes,
    /// False
    No,
    /// true if the parent is true, false if the parent is false
    InheritFromParent,
    /// The default for this decision
    Default,
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
/// The permissions for the current context
pub struct Permissions<T: RuntimePermissionControl> {
    /// The "default" (not the default default) for all people who don't have anything extra defined
    pub all: T,
    /// Permission overwrites for specific people, this overwrites both the "default" and role specification
    pub specific_people: Vec<(UserID, T)>,
    /// Permission overwrites for specific roles, this overwrites only the "default"
    pub specific_roles: Vec<(String, T)>,
}
// pub struct ChannelPermissions {
//     pub all: RawChannelPermissions,
//     pub specific_people: Vec<(UserID, RawChannelPermissions)>,
//     pub specific_roles: Vec<(String, RawChannelPermissions)>,
// }
#[allow(clippy::result_unit_err)]
/// A trait for easier permission management
pub trait RuntimePermissionControl {
    /// Get all permission names
    fn get_available_permissions(&self) -> Vec<String>;
    /// Get the decision for the given permission name
    fn get_permission_decision(&self, name: &str)
    -> Option<PermissionDecision>;
    /// Set the given permission to the given decision
    /// 
    /// # Errors
    /// When the permission name does not exist
    fn set_permission_decision(
        &mut self,
        name: &str,
        value: PermissionDecision,
    ) -> Result<(), ()>;
}
impl RuntimePermissionControl for RawChatPermissions {
    fn get_available_permissions(&self) -> Vec<String> {
        [
            "is_admin",
            "can_send_messages",
            "is_chat_visible",
            "can_react_to_messages",
            "can_add_reactions_to_messages",
            "can_delete_any_content",
            "can_use_pin_feature",
            "can_upload_attachments",
            "can_create_polls",
            "can_vote_on_polls",
        ]
        .into_value()
    }
    fn get_permission_decision(
        &self,
        name: &str,
    ) -> Option<PermissionDecision> {
        Some(match name {
            "is_admin" => self.is_admin,
            "can_send_messages" => self.can_send_messages,
            "is_chat_visible" => self.is_chat_visible,
            "can_react_to_messages" => self.can_react_to_messages,
            "can_add_reactions_to_messages" => {
                self.can_add_reactions_to_messages
            }
            "can_delete_any_content" => self.can_delete_any_content,
            "can_use_pin_feature" => self.can_use_pin_feature,
            "can_upload_attachments" => self.can_upload_attachments,
            "can_create_polls" => self.can_create_polls,
            "can_vote_on_polls" => self.can_vote_on_polls,
            _ => return None,
        })
    }
    fn set_permission_decision(
        &mut self,
        name: &str,
        value: PermissionDecision,
    ) -> Result<(), ()> {
        match name {
            "is_admin" => self.is_admin = value,
            "can_send_messages" => self.can_send_messages = value,
            "is_chat_visible" => self.is_chat_visible = value,
            "can_react_to_messages" => self.can_react_to_messages = value,
            "can_add_reactions_to_messages" => {
                self.can_add_reactions_to_messages = value;
            }
            "can_delete_any_content" => self.can_delete_any_content = value,
            "can_use_pin_feature" => self.can_use_pin_feature = value,
            "can_upload_attachments" => self.can_upload_attachments = value,
            "can_create_polls" => self.can_create_polls = value,
            "can_vote_on_polls" => self.can_vote_on_polls = value,
            _ => return Err(()),
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
/// Permissions that aren't associated with any roles or people in regards to text chats
pub struct RawChatPermissions {
    /// If set to true, the selected is allowed to edit the permissions of the current context and as such has automatically all permissions set to true for itself
    pub is_admin: PermissionDecision,
    /// If the selected can send messages
    pub can_send_messages: PermissionDecision,
    /// If the selected can see the given context
    pub is_chat_visible: PermissionDecision,
    /// If the selected is allowed to add to the same reaction that are already on a message
    pub can_react_to_messages: PermissionDecision,
    /// If the selected is allowed to add new messages to a message
    pub can_add_reactions_to_messages: PermissionDecision,
    /// If the selected is allowed to delete any messages, remove the embedded content, or reactions of anyone
    pub can_delete_any_content: PermissionDecision,
    /// If the selected is allowed to pin/unpin messages
    pub can_use_pin_feature: PermissionDecision,
    /// If the selected is allowed to upload files/images. This may get split in the future for
    pub can_upload_attachments: PermissionDecision,
    /// If the selected is allowed to create polls (selectable list)
    pub can_create_polls: PermissionDecision,
    /// If the selected is allowed to vote in created polls (They can still see the results)
    pub can_vote_on_polls: PermissionDecision,
    /// If the selected is allowed to ignore the set slow mode
    pub can_bypass_slow_mode: PermissionDecision,
}
#[allow(clippy::struct_field_names)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
/// Permissions that aren't associated with any roles or people in regards to categories
pub struct RawChannelPermissions {
    /// Create, manage, and delete any channel.
    /// This means all other permissions in this category are also set to true
    /// Plus [`is_admin`](RawChatPermissions::is_admin) is also set to true
    can_manage_channels: PermissionDecision,
    /// If the selected is allowed to reorder channels
    can_reorder_channels: PermissionDecision,
    /// If the selected is allowed to rename any channel
    can_rename_channels: PermissionDecision,
}
impl RuntimePermissionControl for RawChannelPermissions {
    fn get_available_permissions(&self) -> Vec<String> {
        ["can_manage_channels", "can_reorder_channels", "can_rename_channels"]
            .into_value()
    }

    fn get_permission_decision(
        &self,
        name: &str,
    ) -> Option<PermissionDecision> {
        Some(match name {
            "can_manage_channels" => self.can_manage_channels,
            "can_reorder_channels" => self.can_reorder_channels,
            "can_rename_channels" => self.can_rename_channels,
            _ => return None,
        })
    }

    fn set_permission_decision(
        &mut self,
        name: &str,
        value: PermissionDecision,
    ) -> Result<(), ()> {
        match name {
            "can_manage_channels" => self.can_manage_channels = value,
            "can_reorder_channels" => self.can_reorder_channels = value,
            "can_rename_channels" => self.can_rename_channels = value,
            _ => return Err(()),
        }
        Ok(())
    }
}
