use super::{
    parse_response,
    responses::{LongPollResponse, LongPollSession},
    Error, Result, VkError,
};
use serde::Serialize;
use serde_json::Value;
use std::sync::Arc;
use tokio::sync::Mutex;
use urlencoding::encode;

/// A [`RequestBuilder`] responsible for establishing connections to [VK Long Poll](https://dev.vk.com/en/api/bots-long-poll/getting-started)
/// and sending method requests to the VK API.
///
/// This struct holds your `access_token` and `group_id` obtained from VK.
/// For more information about how to obtai an access token, see
/// [official documentation](https://dev.vk.com/en/api/community-messages/getting-started#Getting%20the%20Access%20Key%20in%20Community%20Settings).
#[derive(Clone, Debug)]
pub struct RequestBuilder {
    client: reqwest::Client,
    access_token: String,
    _ts: Arc<Mutex<Option<String>>>,
    _session: Arc<Mutex<Option<LongPollSession>>>,
}

pub const VK: &'static str = "https://api.vk.com/method";
pub const WAIT_TIME: u8 = 25;
pub const VERSION: &'static str = "5.199";

macro_rules! request {
    ($method:ident) => {
        #[doc = concat!("Sends a `", stringify!($method), "` request using [reqwest] library to accomplish that.")]
        pub async fn $method<T: Serialize + Send>(
            &self,
            url: &str,
            method: &str,
            query: &[u8],
            body: T,
        ) -> Result<Value> {
            // This is totally fine!!! "itoa" library guarantee that it will return valid utf8,
            // hence it's safe to use "unsafe" block here!!! It will make this code blazingly fast!
            #[cfg(feature = "unsafe")]
            let query = unsafe { std::str::from_utf8_unchecked(query) };

            #[cfg(not(feature = "unsafe"))]
            let query = std::str::from_utf8(query).unwrap();

            let query = encode(query).replace("%3D", "=").replace("%26", "&");

            let response = self
                .client
                .$method(if method.is_empty() {
                    format!("{}?{}v={}", url, query, VERSION)
                } else {
                    format!("{}/{}?{}v={}", url, method, query, VERSION)
                })
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
    pub fn new(access_token: impl Into<String>) -> Self {
        RequestBuilder {
            client: reqwest::Client::new(),
            access_token: access_token.into(),
            _ts: Arc::new(Mutex::new(None)),
            _session: Arc::new(Mutex::new(None)),
        }
    }

    pub async fn update_session(&self, new_session: LongPollSession) {
        *self._session.lock().await = Some(new_session)
    }

    pub async fn update_ts(&self, new_ts: String) {
        *self._ts.lock().await = Some(new_ts);
    }

    pub(crate) async fn get_long_poll_server(&self, group_id: u64) -> Result<LongPollSession> {
        let response = parse_response!(
            self.post(
                VK,
                "groups.getLongPollServer",
                format!("group_id={}&", group_id).as_bytes(),
                {}
            )
            .await?,
            LongPollSession
        )?;

        Ok(response)
    }

    pub async fn get_group_id(&self) -> Result<u64> {
        let response = self.post(VK, "groups.getById", b"", {}).await?;
        let group_id = response["response"]["groups"][0]
            .get("id")
            .unwrap()
            .as_u64()
            .unwrap();

        Ok(group_id)
    }

    pub async fn build_long_poll_request(&self, group_id: u64) -> Result<LongPollResponse<Value>> {
        let mut prev_ts = self._ts.lock().await;

        let mut session_guard = self._session.lock().await;
        if session_guard.is_none() {
            let new_session = self.get_long_poll_server(group_id).await?;
            *session_guard = Some(new_session);
        }

        let longpoll = session_guard.as_ref().unwrap();
        let ts = prev_ts.as_ref().unwrap_or(&longpoll.ts);

        let query = format!(
            "act=a_check&key={}&ts={}&wait={}",
            longpoll.key, ts, WAIT_TIME
        );

        let response = self
            .post(&longpoll.server, "", query.as_bytes(), {})
            .await?;

        let mut response = parse_response!(response, LongPollResponse<Value>)?;

        if let Some(ts) = response.ts.take() {
            *prev_ts = Some(ts);
            drop(prev_ts);
        }

        match response.failed {
            Some(1) => Err(Error::EventsOutdated {
                new_ts: response.ts.unwrap(),
            }),
            Some(2) => Err(Error::KeyExpired),
            Some(3) => Err(Error::InformationLost),
            _ => Ok(response),
        }
    }

    request!(post);
    request!(get);
}
