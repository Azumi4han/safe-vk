use crate::{parse_response, responses::events::User, Method};
use serde::Deserialize;
use std::future::IntoFuture;

/// Represents the result of sending a message. If the `peer_ids` parameter is provided,
/// the method returns an array of these objects.
#[derive(Deserialize, Method, Debug)]
#[method_path("messages.send")]
#[optional]
pub struct SendMessage(pub Option<Vec<_SendMessage>>);

#[derive(Deserialize, Debug)]
pub struct _SendMessage {
    /// The ID of the destination.
    pub peer_id: i64,
    /// The ID of the message within the conversation.
    pub message_id: i32,
    /// The ID of the message within the conversation.
    pub conversation_message_id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// An error message if the message was not delivered to the recipient.
    pub error: Option<String>,
}

// Doesn't return anything
#[derive(Deserialize, Method, Debug)]
#[method_path("messages.edit")]
#[optional]
pub struct EditMessage(pub i8);

#[derive(Deserialize, Method, Debug)]
#[method_path("messages.sendMessageEventAnswer")]
pub struct SendMessageEventAnswer(pub i8);

/// Contains counts, participants, chat restrictions, profiles, and groups data
#[derive(Deserialize, Method, Debug)]
#[method_path("messages.getConversationMembers")]
pub struct GetConversationMembers {
    /// The total number of conversation members
    pub count: u64,
    /// A list of conversation members
    pub items: Vec<Member>,
    /// Restrictions applied to the conversation
    pub chat_restrictions: Option<ChatRestrictions>,
    /// List of users profiles
    pub profiles: Vec<User>,
    /// List of groups involved in the conversation
    pub groups: Vec<Group>,
}

/// Represents an individual member within the conversation
#[derive(Debug, Deserialize)]
pub struct Member {
    /// The member's identifier
    pub member_id: i32,
    /// Identifier of the user who invited the member
    pub invited_by: i32,
    /// The date and time the member was added to the conversation, in Unix Timestamp format
    pub join_date: u64,
    /// Indicates whether the member is an administrator.
    #[serde(default)]
    pub is_admin: bool,
    /// Indicates whether the member has the ability to remove other members
    /// This field may not be present if the member is a bot, a detail not explicitly mentioned in the VK API documentation
    #[serde(default)]
    pub can_kick: bool,
}

/// Information regarding the chat's administrative restrictions
#[derive(Debug, Deserialize)]
pub struct ChatRestrictions {
    /// True if only administrators can promote users to admin status
    pub admins_promote_users: bool,
    /// True if only administrators can edit the chat info
    pub only_admins_edit_info: bool,
    /// True if only administrators can edit the pinned message
    pub only_admins_edit_pin: bool,
    /// True if only administrators can invite new users to the chat
    pub only_admins_invite: bool,
    /// True if only administrators can remove users from the chat
    pub only_admins_kick: bool,
}

/// Represents a group within the conversation
#[derive(Debug, Deserialize)]
pub struct Group {
    /// The group's identifier
    pub id: i32,
    /// The name of the group
    pub name: String,
    /// The group's screen name or alias
    pub screen_name: String,
    /// Indicates the group's closed status with an integer (likely 0 for open, 1 for closed)
    pub is_closed: i32,
    /// The type of the group (e.g., "group")
    #[serde(rename = "type")]
    pub group_type: String,
    /// URL to the group's photo of size 50x50 pixels
    pub photo_50: String,
    /// URL to the group's photo of size 100x100 pixels
    pub photo_100: String,
    /// URL to the group's photo of size 200x200 pixels
    pub photo_200: String,
}
