use crate::{
    parse_response,
    traits::{Method, Request},
    util::{Error, NdArray, Result},
    Ctx, LongPollResponse, Members, RequestBuilder, User,
};
use serde::Serialize;
use serde_json::json;
use std::sync::Arc;
use tokio::sync::RwLock;

const VK: &'static str = "https://api.vk.com/method";

#[derive(Debug, Clone)]
pub struct Methods {
    pub request: RequestBuilder,
    pub context: Arc<RwLock<Ctx>>,
}

impl Method for Methods {
    fn new(token: String) -> Self {
        Methods {
            request: RequestBuilder::new(token),
            context: Arc::new(RwLock::new(Ctx::default())),
        }
    }

    async fn keyboard<T: Serialize, N: NdArray<T>>(
        &self,
        message: &str,
        one_time: bool,
        inline: bool,
        buttons: N,
    ) -> Result<()> {
        let state = self.context.read().await;
        let dim_1 = buttons.shape().dims()[1];
        let dim_2 = buttons.shape().dims()[0];

        // Ensure that the first dimension (dim_1) is not greater than 5
        // This is to enforce a maximum shape of 5x1 for the array
        // Fore more info: https://dev.vk.com/ru/api/bots/development/keyboard
        if dim_1 > 5 {
            return Err(Error::DimOutOfRange {
                shape: buttons.shape(),
                dim: dim_1 as i32,
            });
        } else if dim_2 > 10 {
            return Err(Error::DimOutOfRange {
                shape: buttons.shape(),
                dim: dim_2 as i32,
            });
        }

        let button = json!({
            "one_time": one_time,
            "inline": inline,
            "buttons": json!(buttons.slice()),
        });

        for update in &state.updates {
            let button = serde_json::to_string(&button).unwrap();
            self.request
                .post(
                    VK,
                    "messages.send",
                    &[
                        ("message", message.to_string()),
                        (
                            "peer_id",
                            update.object.message.as_ref().unwrap().peer_id.to_string(),
                        ),
                        ("keyboard", button),
                        ("random_id", String::from("0")),
                    ],
                    {},
                )
                .await?;
        }
        Ok(())
    }

    async fn event_answer<
        'de,
        T: Serialize,
        A: serde::de::DeserializeOwned + PartialEq + Serialize,
    >(
        &self,
        event_data: T,
        payload: A,
    ) -> Result<Option<A>> {
        let context = self.context().await;

        for update in &context.updates {
            if let Some(data) = &update.object.payload {
                let deserialized_payload = serde_json::from_value::<A>(data.clone())?;

                if deserialized_payload != payload {
                    return Ok(None);
                }

                let event_data = serde_json::to_string(&event_data).unwrap();
                // Safe unwraps here, because if response has object `payload` then VK api
                // guarantee that it will contain `user_id` field and `peer_id`
                let res = self
                    .request
                    .post(
                        VK,
                        "messages.sendMessageEventAnswer",
                        &[
                            ("event_data", event_data),
                            ("user_id", update.object.user_id.unwrap().to_string()),
                            (
                                "event_id",
                                update.object.event_id.as_ref().unwrap().to_string(),
                            ),
                            ("peer_id", update.object.peer_id.unwrap().to_string()),
                        ],
                        {},
                    )
                    .await?;

                if let Ok(response) = parse_response!(res, crate::EventAnswer) {
                    if response.get_status().is_ok() {
                        return Ok(Some(payload));
                    }
                }
            }
        }
        // Return None if updates doesn't have payload
        Ok(None)
    }

    async fn reply(&self, message: &str) {
        let state = self.context.read().await;
        for update in &state.updates {
            // Using match to extract `peer_id` from the update. According to the VK API
            // documentation, `peer_id` is always present in `message_event` and `message_new` types
            let peer_id = match update.update_type.as_str() {
                "message_event" => update.object.peer_id.unwrap(),
                "message_new" => update.object.message.as_ref().unwrap().peer_id,
                _ => panic!("No peer_id found"),
            };

            self.request
                .post(
                    VK,
                    "messages.send",
                    &[
                        ("message", message),
                        ("peer_id", &peer_id.to_string()),
                        ("random_id", "0"),
                    ],
                    {},
                )
                .await
                .expect("Failed to send/get request from `messages.send`");
        }
    }

    async fn long_poll(&self, group_id: u32) -> LongPollResponse {
        let response = self
            .request
            .post(
                VK,
                "groups.getLongPollServer",
                &[("group_id", &group_id.to_string())],
                {},
            )
            .await
            .expect("Unable to get request");

        parse_response!(response, LongPollResponse)
            .expect("Unable to parse `groups.getLongPollServer` response")
    }

    async fn connect(&self, server: &str, key: String, ts: String, wait: usize) -> Ctx {
        let response = self
            .request
            .post(
                server,
                "",
                &[
                    ("act", "a_check"),
                    ("key", &key),
                    ("ts", &ts),
                    ("wait", &wait.to_string()),
                ],
                {},
            )
            .await
            .expect("Unable to get request");

        parse_response!(response, Ctx).expect("Unable to parse longPoll request")
    }

    async fn get_users(&self, user_ids: &[i32]) -> Result<Vec<User>> {
        let serialize = serde_json::to_string(&user_ids[0])?;
        let response = self
            .request
            .post(VK, "users.get", &[("user_ids", &serialize)], {})
            .await
            .unwrap();
        Ok(parse_response!(response, Vec<User>).expect("Unable to parse `users.get` response"))
    }

    /// This function retrieves a list of participants in the conversation
    async fn get_members(
        &self,
        offset: Option<u16>,
        count: Option<u16>,
        extended: bool,
    ) -> Result<Members> {
        let context = self.context().await;
        for update in &context.updates {
            if let Some(message) = &update.object.message {
                let mut params = vec![("peer_id", message.peer_id.to_string())];

                if extended {
                    params.push(("extended", String::from("1")));
                } else {
                    params.push(("extended", String::from("0")));
                }

                if let Some(offset_val) = offset {
                    params.push(("offset", offset_val.to_string()));
                }
                if let Some(count_val) = count {
                    params.push(("count", count_val.to_string()));
                }

                let res = self
                    .request
                    .post(VK, "messages.getConversationMembers", &params, {})
                    .await?;

                return Ok(parse_response!(res, Members)?);
            }
        }
        Err(Error::NoContent {
            from: "getConversationMembers",
        })
    }

    fn custom_request(&self) -> &RequestBuilder {
        &self.request
    }

    async fn context(&self) -> tokio::sync::RwLockReadGuard<'_, Ctx> {
        self.context.read().await
    }
}
