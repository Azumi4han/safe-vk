use crate::{parse_response, responses::attachment::Photo, Method};
use serde::Deserialize;
use std::future::IntoFuture;

/// Represents the result of sending a message. If the `peer_ids` parameter is provided,
/// the method returns an array of these objects.
#[derive(Deserialize, Method, Debug)]
#[method_path("photos.getMessagesUploadServer")]
pub struct GetMessagesUploadServer {
    pub album_id: i32,
    pub upload_url: String,
    pub user_id: i64,
    pub group_id: u32,
}

#[derive(Deserialize, Method, Debug)]
#[method_path("photos.saveMessagesPhoto")]
pub struct SaveMessagesPhoto(pub Vec<Photo>);
