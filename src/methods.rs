use crate::{
    responses::{Ctx, LongPollResponse},
    traits::{Method, Request},
    RequestBuilder,
};
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

    // async fn keyboard(&self, keys: &[&str]) {
    //     let state = self.context.read().await;
    //     for update in &state.updates {
    //         let keyboard = json!({
    //             "one_time": false,
    //             "buttons": [[
    //                 {
    //                     "action": {
    //                        "type": "text",
    //                         "payload": "{\"button\": \"1\"}",
    //                         "label": "press me"
    //                     },
    //                     "color": "primary"
    //                 }
    //             ]]
    //         });
    //
    //         let keyboard_string = serde_json::to_string(&keyboard).unwrap();
    //
    //         let res = self
    //             .request
    //             .post(
    //                 VK,
    //                "messages.send",
    //                 [
    //                     ("keyboard", keyboard_string),
    //                     ("message", String::from("noob")),
    //                     (
    //                         "peer_id",
    //                         update.object.message.as_ref().unwrap().peer_id.to_string(),
    //                     ),
    //                     ("random_id", String::from("0")),
    //                 ],
    //                 {},
    //             )
    //             .await
    //             .expect("Unable to get request");
    //     }
    // }

    async fn reply(&self, message: &str) {
        let state = self.context.read().await;
        for update in &state.updates {
            self.request
                .post(
                    VK,
                    "messages.send",
                    &[
                        ("message", message),
                        (
                            "peer_id",
                            &update.object.message.as_ref().unwrap().peer_id.to_string(),
                            // Safe unwrap here
                        ),
                        ("random_id", "0"),
                    ],
                    {},
                )
                .await
                .expect("Unable to get request");
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

        crate::parse_response!(response, LongPollResponse)
            .expect("Unable to parse longPoll request")
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

        crate::parse_response!(response, Ctx).expect("Unable to parse longPoll request")
    }

    fn custom_request(&self) -> &RequestBuilder {
        &self.request
    }

    async fn context(&self) -> tokio::sync::RwLockReadGuard<'_, Ctx> {
        self.context.read().await
    }
}
