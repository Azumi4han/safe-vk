use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct GetMessagesUploadServer {
    pub album_id: i32,
    pub upload_url: String,
    pub user_id: i32,
    pub group_id: i32,
}

#[derive(Deserialize, Debug)]
pub struct UploadImage {
    pub hash: String,
    pub photo: String,
    pub server: i32,
}
