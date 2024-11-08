use reqwest::{
    multipart::{Form, Part},
    Client,
};
use safe_vk::{
    api::{CtxAbstraction, MethodBuilder},
    parse_response,
    responses::attachment::Photo,
    Result,
};
use serde::Deserialize;
use std::future::Future;

#[derive(Deserialize, Debug)]
pub struct UploadImage {
    hash: String,
    photo: String,
    server: i32,
}

pub trait CustomFunctions: CtxAbstraction {
    fn upload_image(
        &self,
        image: Vec<u8>,
        filename: &str,
    ) -> impl Future<Output = Result<Vec<Photo>>>;

    fn send_message(&self, message: &str) -> impl Future<Output = Result<()>>;
}

impl CustomFunctions for MethodBuilder {
    async fn upload_image(&self, image: Vec<u8>, filename: &str) -> Result<Vec<Photo>> {
        let upload_server = self.get_messages_upload_server().await?;

        let file_type = filename.rsplit_once('.').map(|(_, ext)| ext).unwrap_or("");
        let mime_type = format!("image/{}", file_type);

        let image_part = Part::stream(image)
            .file_name(filename.to_string())
            .mime_str(&mime_type)
            .unwrap();

        let client = Client::new();
        let form = Form::new().part("photo", image_part);

        let response = client
            .post(&upload_server.upload_url)
            .multipart(form)
            .send()
            .await
            .unwrap()
            .json::<serde_json::Value>()
            .await
            .unwrap();

        let parsed = parse_response!(response, UploadImage).unwrap();

        self.save_messages_photo()
            .photo(&parsed.photo)
            .server(parsed.server)
            .hash(&parsed.hash)
            .await
    }

    async fn send_message(&self, message: &str) -> Result<()> {
        self.send().random_id(0).message(message).await.map(|_| ())
    }
}
