use super::attachment::Attachment;
use crate::Keyboard;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Message<T = ()> {
    pub client_info: ClientInfo,
    pub message: PersonalMessage<T>,
}

/// Represents client information received in the `client_info` field of the message_new event.
#[derive(Debug, Deserialize, Clone)]
pub struct ClientInfo {
    /// Array of supported button actions.
    pub button_actions: Vec<String>,
    /// Indicates whether keyboards are supported by the client.
    pub keyboard: bool,
    /// Indicates whether inline keyboards are supported by the client.
    pub inline_keyboard: bool,
    /// Indicates whether carousels are supported by the client.
    pub carousel: bool,
    /// The ID of the language used.
    pub lang_id: i32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ReplyMessage<T> {
    /// Identifier of the message.
    pub id: i32,
    /// Time the message was sent in Unixtime.
    pub date: i64,
    /// Destination identifier.
    pub peer_id: i64,
    /// Sender identifier.
    pub from_id: i32,
    /// Message text.
    pub text: String,
    /// Media attachments in the message (photos, links, etc.).
    pub attachments: Vec<Attachment<T>>,
    /// Unique automatically increasing number for all messages with this peer.
    pub conversation_message_id: i32,
    /// Service field for bot messages (payload).
    pub payload: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct PersonalMessage<T> {
    /// Identifier of the message.
    pub id: i32,
    /// Time the message was sent in Unixtime.
    pub date: i64,
    /// Destination identifier.
    pub peer_id: i64,
    /// Sender identifier.
    pub from_id: i32,
    /// Message text.
    pub text: String,
    /// Identifier used when sending a message. Returned only for outgoing messages.
    /// Is optional, because reply messages doesn't have it
    pub random_id: i32,
    /// Arbitrary parameter for working with transition sources.
    #[serde(rename = "ref")]
    pub reference: Option<String>,
    /// Arbitrary parameter for working with transition sources.
    pub ref_source: Option<String>,
    /// Media attachments in the message (photos, links, etc.).
    pub attachments: Vec<Attachment<T>>,
    /// True if the message is marked as important.
    pub important: bool,
    /// Information about the location.
    pub geo: Option<Geo>,
    /// Service field for bot messages (payload).
    pub payload: Option<String>,
    /// Keyboard object for bots.
    pub keyboard: Option<Keyboard<T>>,
    /// Array of forwarded messages (if any).
    pub fwd_messages: Vec<PersonalMessage<T>>,
    /// The message in reply to which the current one was sent.
    pub reply_message: Option<ReplyMessage<T>>,
    /// Information about a service action with the chat.
    pub action: Option<Action>,
    /// Identifier of the user (community administrator) who sent this message. Only for community messages.
    pub admin_author_id: Option<i32>,
    /// Unique automatically increasing number for all messages with this peer.
    pub conversation_message_id: i32,
    /// Indicates if this message is cropped for the bot.
    pub is_cropped: Option<bool>,
    /// Number of participants.
    pub members_count: Option<i32>,
    /// Date when the message was updated in Unixtime.
    pub update_time: Option<i64>,
    /// Indicates if the embedded voice message has been listened to by you.
    pub was_listened: Option<bool>,
    /// Date when the message was pinned in Unixtime.
    pub pinned_at: Option<i64>,
    /// String for matching user Notify and VKontakte.
    pub message_tag: Option<String>,
    /// Flag indicates if the user is mentioned in this message.
    pub is_mentioned_user: Option<bool>,
}

/// Represents geographical location information.
#[derive(Debug, Deserialize, Clone)]
pub struct Geo {
    /// Type of place.
    #[serde(rename = "type")]
    pub geo_type: String,
    /// Place coordinates.
    pub coordinates: Coordinates,
    /// Description of the place (if added).
    pub place: Option<Place>,
}

/// Represents coordinates.
#[derive(Debug, Deserialize, Clone)]
pub struct Coordinates {
    /// Geographic latitude.
    pub latitude: f64,
    /// Geographic longitude.
    pub longitude: f64,
}

/// Represents a place.
#[derive(Debug, Deserialize, Clone)]
pub struct Place {
    /// Place identifier (if assigned).
    pub id: Option<i64>,
    /// Name of the place (if assigned).
    pub title: Option<String>,
    /// Geographic latitude.
    pub latitude: f64,
    /// Geographic longitude.
    pub longitude: f64,
    /// Date of creation (if assigned).
    pub created: Option<i64>,
    /// URL of the icon image.
    pub icon: Option<String>,
    /// Name of the country.
    pub country: Option<String>,
    /// Name of the city.
    pub city: Option<String>,
}

/// Represents a message action.
#[derive(Debug, Deserialize, Clone)]
pub struct Action {
    /// Type of action.
    #[serde(rename = "type")]
    pub action_type: String,
    /// Identifier of the user or email.
    pub member_id: Option<i64>,
    /// Name of the conversation (for service messages).
    pub text: Option<String>,
    /// Email invited or excluded (for service messages).
    pub email: Option<String>,
    /// Chat cover image.
    pub photo: Option<Photo>,
}

/// Represents photo sizes for chat cover.
#[derive(Debug, Deserialize, Clone)]
pub struct Photo {
    pub photo_50: String,
    pub photo_100: String,
    pub photo_200: String,
}
