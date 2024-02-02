use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize, Debug, PartialEq)]
pub struct Ctx {
    pub ts: String,
    pub updates: Vec<Update>,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct Update {
    pub event_id: String,
    // Bot group ID                         (bot)
    pub group_id: i32,
    // Update object                        (anything)
    pub object: UpdateObject,
    #[serde(rename = "type")]
    pub update_type: String,
    // VK api version                       (const / DO NOT CHANGE)
    pub v: String,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct UpdateObject {
    // Information about user               (message)
    pub client_info: Option<ClientInfo>,
    // Message object                       (message)
    pub message: Option<Message>,
    // Conversation message ID              (message)
    pub cmid: Option<i32>,
    // unique ID for your button            (keyboard)
    pub event_id: Option<String>,
    // Unique ID for conversation / groups  (message)
    pub peer_id: Option<i32>,
    // Who wrote message                    (message)
    pub user_id: Option<i32>,
    // Your custom payload for your button  (keyboard)
    pub payload: Option<Value>,
    // Reacted id                           (message)
    pub reacted_id: Option<i32>,
    // Reaction id                          (message)
    pub reaction_id: Option<i32>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct ClientInfo {
    pub button_actions: Vec<String>,
    pub carousel: bool,
    pub inline_keyboard: bool,
    pub keyboard: bool,
    pub lang_id: usize,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Message {
    pub attachments: Vec<Attachment>,
    pub conversation_message_id: i32,
    pub date: i32,
    pub from_id: i32,
    pub fwd_message: Option<Vec<String>>,
    pub id: i32,
    pub important: bool,
    pub is_hidden: bool,
    pub is_unavailable: bool,
    pub out: i32,
    pub peer_id: i32,
    pub random_id: i32,
    pub text: String,
    pub version: u64,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Attachments {
    photo: Option<Vec<Attachment>>,
    video: Option<Vec<Attachment>>,
    #[serde(rename = "type")]
    attachment_type: String,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Attachment {
    #[serde(rename = "type")]
    pub attachment_type: String,
    pub photo: Option<Photo>,
    pub video: Option<Video>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Photo {
    pub access_key: String,
    pub album_id: i32,
    pub date: u32,
    pub has_tags: bool,
    pub id: i32,
    pub owner_id: i32,
    pub sizes: Vec<AttachmentSize>,
    pub text: String,
    pub web_view_token: String,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct AttachmentSize {
    height: usize,
    #[serde(rename = "type")]
    size_type: String,
    url: String,
    width: usize,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct Video {
    access_key: String,
    can_add: i32,
    content_restricted: i32,
    date: i32,
    duration: i32,
    height: i32,
    id: i32,
    image: Vec<Image>,
    is_private: i32,
    owner_id: i32,
    response_type: String,
    restriction: Restriction,
    title: String,
    track_code: String,
    #[serde(rename = "type")]
    video_type: String,
    views: i32,
    width: i32,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct Image {
    height: i32,
    url: String,
    width: i32,
    with_padding: i32,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct Restriction {
    blur: i32,
    can_play: i32,
    can_preview: i32,
    card_icon: Vec<Icon>,
    disclaimer_type: i32,
    icon_name: String,
    list_icon: Vec<Icon>,
    text: String,
    title: String,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct Icon {
    height: i32,
    url: String,
    width: i32,
}
