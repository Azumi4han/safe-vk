use super::{
    responses::{LongPollResponse, LongPollSession},
    Error, Result, VkError,
};
use serde::Serialize;
use serde_json::Value;
use tokio::sync::Mutex;

/// A [`RequestBuilder`] responsible for establishing connections to [VK Long Poll](https://dev.vk.com/en/api/bots-long-poll/getting-started)
/// and sending method requests to the VK API.
///
/// This struct holds your `access_token` and `group_id` obtained from VK.
/// For more information about how to obtai an access token, see
/// [official documentation](https://dev.vk.com/en/api/community-messages/getting-started#Getting%20the%20Access%20Key%20in%20Community%20Settings).
pub struct RequestBuilder {
    client: reqwest::Client,
    access_token: String,
    group_id: u32,
    _ts: Mutex<usize>,
}

pub const VK: &'static str = "https://api.vk.com/method";
pub const VERSION: &'static str = "5.199";

macro_rules! request {
    ($method:ident) => {
        #[doc = concat!("Sends a `", stringify!($method), "` request using [reqwest] library to accomplish that.")]
        pub async fn $method<T: Serialize + Send, A: Serialize + Send + Sized>(
            &self,
            url: &str,
            method: &str,
            query: A,
            body: T,
        ) -> Result<Value> {
            let response = self
                .client
                .$method(&if method.is_empty() {
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

            if let Some(err) = json.get("error") {
                return Err(Error::VkApi(VkError::from_vk_error_json(err)));
            }

            Ok(json)
        }
    };
}

impl RequestBuilder {
    /// Creates a new instance of [RequestBuilder]
    pub fn new(access_token: &str, group_id: u32) -> Self {
        RequestBuilder {
            client: reqwest::Client::new(),
            access_token: access_token.to_string(),
            group_id,
            _ts: Mutex::new(0),
        }
    }
    /// Build a long poll request for retriving an udpates from VK
    /// ```rust
    /// use safe_vk::{SafeVk, RequestBuilder};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let request = RequestBuilder::new("my secret token", 1234);
    ///  
    ///     let _ = request.build_long_poll_request().await.unwrap();
    /// }
    ///
    /// ```
    pub async fn build_long_poll_request(&self) -> Result<LongPollResponse<Value>> {
        let response = self
            .post(
                VK,
                "groups.getLongPollServer",
                &[("group_id", self.group_id)],
                {},
            )
            .await?;

        let parsed_response =
            crate::parse_response!(response, LongPollSession).map_err(|e| Error::SerdeJson(e))?;

        // Safe unwrap here, since VK always return a number in "ts" field
        let ts = parsed_response.ts.parse::<usize>().unwrap();
        let prev_ts = self._ts.lock().await;

        let ts = if *prev_ts == ts { ts + 1 } else { ts };

        let response = self
            .post(
                &parsed_response.server,
                "",
                &[
                    ("act", String::from("a_check")),
                    ("key", parsed_response.key),
                    ("ts", ts.to_string()),
                    ("wait", String::from("25")),
                ],
                {},
            )
            .await?;

        drop(prev_ts);

        let parsed_response = crate::parse_response!(response, LongPollResponse<Value>)
            .map_err(|e| Error::SerdeJson(e))?;

        if let Some(ref updates) = parsed_response.updates {
            if updates.len() > 0 {
                let mut local_version = self._ts.lock().await;
                *local_version = ts;
            }
        }

        match parsed_response.failed {
            Some(1) => Err(Error::EventsOutdated {
                new_ts: parsed_response.ts.unwrap(),
            }),
            Some(2) => Err(Error::KeyExpired),
            Some(3) => Err(Error::InformationLost),
            _ => Ok(parsed_response),
        }
    }

    request!(post);
    request!(get);
}
