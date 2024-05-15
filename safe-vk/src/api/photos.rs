use crate::{
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
use std::sync::Arc;

pub struct PhotosBuilder {
    request: Arc<RequestBuilder>,
    values: Vec<(&'static str, String)>,
}

impl PhotosBuilder {
    pub fn new(request: Arc<RequestBuilder>, peer_id: String) -> Self {
        let values = Vec::from([("peer_id", peer_id)]);
        PhotosBuilder { request, values }
    }

    pub async fn get_message_upload(&self) -> Result<GetMessagesUploadServer> {
        let response = self
            .request
            .post(VK, "photos.getMessagesUploadServer", (), &self.values)
            .await?;

        Ok(parse_response!(response, GetMessagesUploadServer)?)
    }

    pub async fn save_message_photo(
        &self,
        photo: String,
        server: i32,
        hash: String,
    ) -> Result<Vec<Photo>> {
        let response = self
            .request
            .post(
                VK,
                "photos.saveMessagesPhoto",
                &[
                    ("photo", photo),
                    ("server", server.to_string()),
                    ("hash", hash),
                ],
                {},
            )
            .await?;

        Ok(parse_response!(response, Vec<Photo>)?)
    }

    pub async fn upload(&self, image: Vec<u8>, filename: impl Into<String>) -> Result<Vec<Photo>> {
        let response = self.get_message_upload().await?;

        let filename = &filename.into();

        let file_type = filename.rsplit_once('.').map(|(_, ext)| ext).unwrap_or("");

        let image = Part::stream(image)
            .file_name(filename.clone())
            .mime_str(&format!("image/{file_type}"))?;

        let response = Client::new()
            .post(response.upload_url)
            .multipart(Form::new().part("photo", image))
            .send()
            .await
            .unwrap()
            .json::<serde_json::Value>()
            .await
            .unwrap();

        let response = parse_response!(response, UploadImage)?;

        let response = self
            .save_message_photo(response.photo, response.server, response.hash)
            .await?;

        Ok(response)
    }
}

impl Ctx<Message> {
    pub fn photos(&self) -> PhotosBuilder {
        let peer_id = self.message.peer_id.to_string();
        PhotosBuilder::new(self.request.clone(), peer_id)
    }
}

impl Ctx<Update> {
    pub fn photos(&self) -> Result<PhotosBuilder> {
        let object = self.object.get("message").unwrap();
        let peer_id = object.get("peer_id").unwrap().to_string();
        Ok(PhotosBuilder::new(self.request.clone(), peer_id))
    }
}
