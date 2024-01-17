use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq)]
pub struct Ctx {
    pub ts: String,
    pub updates: Vec<Update>,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct Update {
    pub event_id: String,
    pub group_id: u64,
    pub object: UpdateObject,
    #[serde(rename = "type")]
    pub update_type: String,
    pub v: String,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct UpdateObject {
    pub client_info: Option<ClientInfo>,
    pub message: Option<Message>,
    pub cmid: Option<i32>,
    pub peer_id: Option<u64>,
    pub reacted_id: Option<u64>,
    pub reaction_id: Option<u64>,
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
    pub conversation_message_id: u64,
    pub date: u64,
    pub from_id: u64,
    pub fwd_message: Option<Vec<String>>,
    pub id: u64,
    pub important: bool,
    pub is_hidden: bool,
    pub is_unavailable: bool,
    pub out: u64,
    pub peer_id: u64,
    pub random_id: u64,
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
    pub id: u64,
    pub owner_id: u64,
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
    date: i64,
    duration: i64,
    height: i32,
    id: i64,
    image: Vec<Image>,
    is_private: i32,
    owner_id: i64,
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
