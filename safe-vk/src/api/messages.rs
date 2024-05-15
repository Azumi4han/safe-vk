//! Functions that interacts with VK API
use crate::{
    extract::{Ctx, Update},
    parse_response,
    responses::{ButtonPressCallback, EventAnswer, Members, Message},
    Error, NdArray, RequestBuilder, Result, VK,
};
use serde_json::{json, Value};
use std::sync::Arc;

pub struct MessageBuilder {
    /// Update data and RequestBuilder
    request: Arc<RequestBuilder>,
    values: Vec<(&'static str, String)>,
}

impl MessageBuilder {
    pub fn new(request: Arc<RequestBuilder>, peer_id: String, message: Option<String>) -> Self {
        let mut values = vec![("random_id", String::from("0")), ("peer_id", peer_id)];

        if message.is_some() {
            values.push(("message", message.unwrap()));
        }

        MessageBuilder { request, values }
    }

    fn iter_to_string(&self, value: &[i32]) -> String {
        value
            .iter()
            .map(|id| id.to_string())
            .collect::<Vec<_>>()
            .join(",")
    }

    pub fn group_id(mut self, group_id: u32) -> Self {
        self.values.push(("group_id", group_id.to_string()));
        self
    }

    pub fn random_id(mut self, random_id: i32) -> Self {
        self.values.push(("random_id", random_id.to_string()));
        self
    }

    pub fn peer_id(mut self, peer_id: i32) -> Self {
        self.values.push(("peer_id", peer_id.to_string()));
        self
    }

    /// User's short page adress, that you can find in url, for example `azumiiii` in
    /// https://vk.com/azumiiii
    pub fn domaim(mut self, user_domain: impl Into<String>) -> Self {
        self.values.push(("domain", user_domain.into()));
        self
    }

    pub fn chat_id(mut self, chat_id: i32) -> Self {
        self.values.push(("chat_id", chat_id.to_string()));
        self
    }

    pub fn user_ids(mut self, user_ids: &[i32]) -> Self {
        let user_ids = self.iter_to_string(user_ids);
        self.values.push(("user_ids", user_ids));
        self
    }

    pub fn attachment(
        mut self,
        media_type: impl Into<String>,
        owner_id: i64,
        media_id: i64,
    ) -> Self {
        self.values.push((
            "attachment",
            format!("{}{}_{}", media_type.into(), owner_id, media_id),
        ));
        self
    }

    pub fn peer_ids(mut self, peer_ids: &[i32]) -> Self {
        let peer_ids = self.iter_to_string(peer_ids);

        self.values.push(("peer_ids", peer_ids));
        // Removing `peer_id`, otherwise it will not work
        self.values.remove(1);
        self
    }

    /// Unique identifier designed to prevent the same message from being sent repeatedly.
    pub fn guid(mut self, guid: i32) -> Self {
        self.values.push(("guid", guid.to_string()));
        self
    }

    pub fn lat(mut self, lat: i8) -> Result<Self> {
        self.values.push(("lat", lat.to_string()));
        Ok(self)
    }

    pub fn long(mut self, long: i8) -> Result<Self> {
        self.values.push(("long", long.to_string()));
        Ok(self)
    }

    /// Replies to a specific message in a private conversation with the bot.
    /// The message ID is required to identify which message to reply to.
    /// ```rust
    /// use safe_vk::{extract::Ctx, responses::Message};
    ///
    /// async fn reply(update: Ctx<Message>) {
    ///     let id = update.message.id;
    ///     update.message_text("Nya!").reply_to(id).send().await.unwrap();
    /// }
    /// ```
    pub fn reply_to(mut self, message_id: i32) -> Self {
        self.values.push(("reply_to", message_id.to_string()));
        self
    }

    /// Forwards messages by their ID. You can specify multiple message IDs.
    /// ```rust
    /// use safe_vk::{extract::Ctx, responses::Message};
    ///
    /// async fn reply(update: Ctx<Message>) {
    ///     update.message_text("Nya!").forward_messages(&[540154586, 1]).send().await.unwrap();
    /// }
    /// ```
    pub fn forward_messages(mut self, ids: &[i32]) -> Self {
        let ids = self.iter_to_string(ids);
        self.values.push(("forward_messages", ids));
        self
    }

    pub fn forward(mut self, object: Value) -> Self {
        self.values
            .push(("forward", serde_json::from_value(object).unwrap()));
        self
    }

    pub fn sticker_id(mut self, id: u32) -> Self {
        self.values.push(("sticker_id", id.to_string()));
        self
    }

    pub fn intent(mut self, intent: impl Into<String>) -> Self {
        self.values.push(("intent", intent.into()));
        self
    }

    pub async fn send(self) -> Result<()> {
        self.request
            .post(VK, "messages.send", self.values, {})
            .await?;

        Ok(())
    }

    pub async fn get_members(
        self,
        offset: Option<u16>,
        count: Option<u16>,
        extended: bool,
    ) -> Result<Members> {
        let mut values = vec![];

        values.extend(self.values);

        if extended {
            values.push(("extended", String::from("1")));
        } else {
            values.push(("extended", String::from("0")));
        }

        if let Some(offset_val) = offset {
            values.push(("offset", offset_val.to_string()));
        }
        if let Some(count_val) = count {
            values.push(("count", count_val.to_string()));
        }

        let response = self
            .request
            .post(VK, "messages.getConversationMembers", &values, {})
            .await?;

        Ok(parse_response!(response, Members)?)
    }
}

impl Ctx<Message> {
    pub fn message(&self) -> MessageBuilder {
        let peer_id = self.message.peer_id.to_string();
        MessageBuilder::new(self.request.clone(), peer_id, None)
    }

    /// Sends message back to conversation
    ///
    /// # Example
    ///
    /// ```rust
    /// use safe_vk::{extract::Ctx, responses::Message, Result};
    ///
    /// async fn message(update: Ctx<Message>) -> Result<()> {
    ///     update.message_text("hello from rust! 🦀").send().await.unwrap();
    ///     Ok(())
    /// }
    /// ```
    pub fn message_text(&self, message: impl Into<String>) -> MessageBuilder {
        let peer_id = self.message.peer_id.to_string();
        MessageBuilder::new(self.request.clone(), peer_id, Some(message.into()))
    }

    pub async fn keyboard<T: serde::Serialize, N: NdArray<T>>(
        &self,
        message: impl Into<String>,
        one_time: bool,
        inline: bool,
        buttons: N,
    ) -> crate::Result<()> {
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

        let keyboard = json!({
            "one_time": one_time,
            "inline": inline,
            "buttons": json!(buttons.slice()),
        });

        let keyboard = serde_json::to_string(&keyboard)?;

        self.request
            .post(
                VK,
                "messages.send",
                &[
                    ("message", message.into()),
                    ("peer_id", self.message.peer_id.to_string()),
                    ("keyboard", keyboard),
                    ("random_id", String::from("0")),
                ],
                {},
            )
            .await?;

        Ok(())
    }
}

impl Ctx<Update> {
    pub fn message(&self) -> Result<MessageBuilder> {
        let object = self.object.get("message").unwrap();
        let peer_id = object.get("peer_id").unwrap().to_string();
        Ok(MessageBuilder::new(self.request.clone(), peer_id, None))
    }

    pub fn message_text(&self, message: impl Into<String>) -> MessageBuilder {
        let object = self.object.get("message").unwrap();
        let peer_id = object.get("peer_id").unwrap().to_string();
        MessageBuilder::new(self.request.clone(), peer_id, Some(message.into()))
    }

    pub async fn keyboard_callback<
        T: serde::Serialize,
        A: serde::de::DeserializeOwned + PartialEq + serde::Serialize,
    >(
        &self,
        callback: T,
        _payload: A,
    ) -> crate::Result<Option<ButtonPressCallback<A>>> {
        if let Ok(valid_data) =
            serde_json::from_value::<ButtonPressCallback<A>>(self.object.clone())
        {
            let event_data = serde_json::to_string(&callback).unwrap();
            // Safe unwraps here, because if response has object `payload` then VK api
            // guarantee that it will contain `user_id` field and `peer_id`
            let res = self
                .request
                .post(
                    VK,
                    "messages.sendMessageEventAnswer",
                    &[
                        ("event_data", event_data),
                        ("user_id", valid_data.user_id.to_string()),
                        ("event_id", valid_data.event_id.to_string()),
                        ("peer_id", valid_data.peer_id.to_string()),
                    ],
                    {},
                )
                .await?;

            if let Ok(response) = parse_response!(res, EventAnswer) {
                if response.get_status().is_ok() {
                    return Ok(Some(valid_data));
                }
            }
        }

        Ok(None)
    }
}
