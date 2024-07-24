use serde::Deserialize;

/// Represents the result of sending a message. If the `peer_ids` parameter is provided,
/// the method returns an array of these objects.
#[derive(Debug, Deserialize)]
pub struct MessageSendResult {
    /// The ID of the destination.
    pub peer_id: i64,
    /// The ID of the message.
    pub message_id: i32,
    /// The ID of the message within the conversation.
    pub conversation_message_id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// An error message if the message was not delivered to the recipient.
    pub error: Option<String>,
}
