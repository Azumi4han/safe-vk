use super::Write;
use crate::{
    api_func, chained_method_fn,
    extract::{Ctx, Update},
    parse_response,
    responses::{
        attachment::Photo,
        events::{GetMessagesUploadServer, UploadImage},
        Message,
    },
    RequestBuilder, Result, VK,
};
use reqwest::{
    multipart::{Form, Part},
    Client,
};
use std::{fmt, future::IntoFuture, sync::Arc};

pub struct PhotosBuilder {
    request: Arc<RequestBuilder>,
    _peer_id: i64,
}

chained_method_fn!(
    MessageUploadServer,
    GetMessagesUploadServer,
    "photos.getMessagesUploadServer",
    peer_id(i64)
);

chained_method_fn!(
    SaveMessagesPhoto,
    Vec<Photo>,
    "photos.saveMessagesPhoto",
    server(i32),
    hash(&str),
    photo(&str)
);

impl PhotosBuilder {
    fn new(request: Arc<RequestBuilder>, _peer_id: i64) -> PhotosBuilder {
        PhotosBuilder { request, _peer_id }
    }

    pub fn get_messages_upload_server(&self) -> MessageUploadServer {
        let request = Arc::clone(&self.request);
        MessageUploadServer::new(request, None)
    }

    pub fn save_messages_photo(&self) -> SaveMessagesPhoto {
        let request = Arc::clone(&self.request);
        SaveMessagesPhoto::new(request, None)
    }

    pub async fn upload_image(&self, image: Vec<u8>, filename: &str) -> Result<Vec<Photo>> {
        let upload_server = self.get_messages_upload_server().await?;

        let file_type = filename.rsplit_once('.').map(|(_, ext)| ext).unwrap_or("");
        let mime_type = format!("image/{}", file_type);

        let image_part = Part::stream(image)
            .file_name(filename.to_string())
            .mime_str(&mime_type)?;

        let client = Client::new();
        let form = Form::new().part("photo", image_part);

        let response = client
            .post(&upload_server.upload_url)
            .multipart(form)
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;

        let parsed = parse_response!(response, UploadImage)?;

        self.save_messages_photo()
            .photo(&parsed.photo)
            .server(parsed.server)
            .hash(&parsed.hash)
            .await
    }
}

impl Ctx<Message> {
    pub fn photos(&self) -> PhotosBuilder {
        let peer_id = self.message.peer_id;
        PhotosBuilder::new(self.request.clone(), peer_id)
    }
}

impl Ctx<Update> {
    pub fn photos(&self) -> Result<PhotosBuilder> {
        let peer_id = self.find_peer_id(&self.object)?;
        Ok(PhotosBuilder::new(self.request.clone(), peer_id))
    }
}
