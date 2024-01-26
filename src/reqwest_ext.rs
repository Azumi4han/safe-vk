use crate::traits::Request;
use serde::Serialize;
use serde_json::Value;

const VERSION: &'static str = "5.199";

#[derive(Clone, Debug)]
pub struct RequestBuilder {
    client: reqwest::Client,
    access_token: String,
}

macro_rules! request {
    ($fn1:ident) => {
        async fn $fn1<T: Serialize + Send, A: Serialize + Send + Sized>(
            &self,
            url: &str,
            method: &str,
            query: A,
            body: T,
        ) -> std::result::Result<Value, reqwest::Error> {
            let response = self
                .client
                .post(&if method.is_empty() {
                    format!("{}?v={}", url, VERSION)
                } else {
                    format!("{}/{}?v={}", url, method, VERSION)
                })
                .query(&query)
                .bearer_auth(&self.access_token)
                .json(&body)
                .send()
                .await?;

            let json: Value = response.json().await?;

            Ok(json)
        }
    };
}

impl Request for RequestBuilder {
    fn new(access_token: String) -> Self {
        let client = reqwest::Client::new();
        RequestBuilder {
            client,
            access_token,
        }
    }

    request!(post);
    request!(get);
}
