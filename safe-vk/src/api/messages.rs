//! Functions that interacts with VK API
use crate::{
    api::Write,
    api_func, builder_methods, chained_method_fn,
    extract::{Ctx, Update},
    parse_response,
    responses::{events::MessageSendResult, EventAnswer, Members, Message},
    Error, NdArray, RequestBuilder, Result, VK,
};
use serde::Serialize;
use std::{fmt, future::IntoFuture, sync::Arc};

chained_method_fn!(
    GetMember,
    Members,
    "messages.getConversationMembers",
    offset(u16),
    count(u16),
    extended(bool),
    fields(&str),
    group_id(u32)
);

chained_method_fn!(
    SendMessage,
    Option<Vec<MessageSendResult>>,
    "messages.send",
    user_id(i32),
    random_id(i32),
    domain(&str),
    chat_id(i32),
    user_ids(&[i32]),
    message(&str),
    guid(i32),
    lat(i8),
    long(i8),
    reply_to(i32),
    forward_messages(&[i32]),
    forward(&[i32]),
    sticker_id(&[u32]),
    group_id(u32),
    //keyboard
    //template
    //payload
    //content_source
    dont_parse_links(bool),
    disable_mentions(bool),
    intent(&str),
    subscribe_id(i32)
);

chained_method_fn!(
    SendMessageEventAnswer,
    Option<EventAnswer>,
    "messages.sendMessageEventAnswer",
    event_id(&str),
    user_id(i32)
);

chained_method_fn!(
    EditMessage,
    Option<()>,
    "messages.edit",
    message(&str),
    lat(&str),
    long(&str),
    keep_forward_messages(bool),
    keep_snippets(bool),
    group_id(u32),
    dont_parse_links(bool),
    disable_mentions(bool),
    message_id(i32),
    conversation_message_id(i32)
);

impl EditMessage {
    pub fn peer_id(mut self, peer_id: i64) -> Self {
        self.query.remove(1);
        self.arg("peer_id", peer_id);
        self
    }

    pub fn attachment(mut self, media_type: &str, owner_id: i64, media_id: i64) -> Self {
        self.arg_fmt(
            "attachment",
            format_args!("{media_type}{owner_id}_{media_id}"),
        );
        self
    }
}

impl SendMessageEventAnswer {
    pub fn peer_id(mut self, peer_id: i64) -> Self {
        // Removing default `peer_id`
        self.query.remove(1);
        self.arg("peer_id", peer_id);
        self
    }

    pub fn event_data<T>(mut self, event: T) -> Self
    where
        T: Serialize,
    {
        self.arg_json("event_data", event);
        self
    }
}

impl SendMessage {
    pub fn peer_id(mut self, peer_id: i64) -> Self {
        // Removing default `peer_id`
        self.query.remove(1);
        self.arg("peer_id", peer_id);
        self
    }

    pub fn peer_ids(mut self, peer_ids: &[i64]) -> Self {
        // Removing default `peer_id`
        self.query.remove(1);
        self.arg("peer_ids", peer_ids);
        self
    }

    pub fn attachment(mut self, media_type: &str, owner_id: i64, media_id: i64) -> Self {
        self.arg_fmt(
            "attachment",
            format_args!("{media_type}{owner_id}_{media_id}"),
        );
        self
    }

    pub fn keyboard<T, N>(mut self, one_time: bool, inline: bool, buttons: N) -> Result<Self>
    where
        T: serde::Serialize,
        N: NdArray<T>,
    {
        let dim1 = buttons.shape().dims()[1];
        let dim2 = buttons.shape().dims()[0];

        // Ensure that the first dimension (dim1) is not greater than 5
        // This is to enforce a maximum shape of 5x1 for the array
        // Fore more info: https://dev.vk.com/ru/api/bots/development/keyboard
        if dim1 > 5 {
            return Err(Error::DimOutOfRange {
                shape: buttons.shape(),
                dim: dim1,
            });
        } else if dim2 > 10 {
            return Err(Error::DimOutOfRange {
                shape: buttons.shape(),
                dim: dim2,
            });
        }

        let keyboard = serde_json::json!({
            "one_time": one_time,
            "inline": inline,
            "buttons": buttons.slice(),
        });

        self.arg_json("keyboard", keyboard);

        Ok(self)
    }
}

use super::Builder;

pub type MessageBuilder = Builder<Message>;

impl Builder<Message> {
    builder_methods!(
        send -> SendMessage,
        edit -> EditMessage,
        send_message_event_answer -> SendMessageEventAnswer,
        members -> GetMember
    );
}

impl Ctx<Message> {
    pub fn messages(&self) -> MessageBuilder {
        let peer_id = self.message.peer_id;
        MessageBuilder::new(self.request.clone(), peer_id)
    }
}

impl Ctx<Update> {
    pub fn messages(&self) -> Result<MessageBuilder> {
        let peer_id = self.find_peer_id(&self.object)?;
        Ok(MessageBuilder::new(self.request.clone(), peer_id))
    }
}
