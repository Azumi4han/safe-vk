use crate::{
    Error, NdArray, Result, __method,
    api::{
        EditMessageMethod, GetConversationMembersMethod, MethodBuilder,
        SendMessageEventAnswerMethod, SendMessageMethod, Write,
    },
};

impl MethodBuilder<SendMessageMethod> {
    __method! {
        /// Sets the recipient's ID.
        ///
        /// `id`: The user ID receiving the message. Can be replaced with `peer_id`.
        fn user_id(id: i32)

        /// Sets a unique identifier to avoid sending the same message more than once.
        ///
        /// `id`: A unique integer identifier, required to prevent duplicate message sending.
        /// Pass `0` if uniqueness check is not required. Any other value ensures uniqueness.
        fn random_id(id: i32)

        /// Sets the text of the message.
        ///
        /// `message`: The message text to be sent.
        /// Max length is 9000 characters. Required if the `attachment` parameter is not set.
        fn message(message: &str)

        /// Sets the short address (domain) of the user.
        ///
        /// `domain`: Short address of the recipient, e.g., "persik_ryzhiy".
        fn domain(domain: &str)

        /// Sets the chat ID for group conversations.
        ///
        /// `chat_id`: The ID of the group conversation where the message will be sent.
        fn chat_id(id: i32)

        /// Sets multiple recipient IDs (comma-separated).
        ///
        /// `ids`: A string of comma-separated user IDs. Max 100 recipients.
        /// Available only for community access tokens.
        fn user_ids(ids: &[i32])

        /// Sets the reply to a specific message ID.
        ///
        /// `message_id`: ID of the message to reply to.
        fn reply_to(message_id: i32)

        /// Forwards messages.
        ///
        /// `forward_messages`: A string of comma-separated message IDs to forward. Max 100 IDs.
        fn forward_messages(forward_messages_ids: &[i32])

        /// Forwards messages using a JSON object.
        ///
        /// `forward`: A JSON object containing details of the forwarded messages, such as owner, peer ID, and message IDs.
        fn forward(forward: &[i32])

        /// Sets the sticker to be sent.
        ///
        /// `sticker_id`: The sticker ID.
        fn sticker_id(sticker_id: &[u32])

        /// Sets the community ID for sending community messages.
        ///
        /// `group_id`: The community (group) ID.
        fn group_id(group_id: u32)

        /// Attaches a message template.
        ///
        /// `template`: A JSON object describing the message template.
        // fn template(template: &str);

        /// Sets additional payload data.
        ///
        /// `payload`: Additional data to include with the message.
        // fn payload(payload: &str);

        /// Sets the source of user content for chatbots.
        ///
        /// `content_source`: A JSON object describing the source of the user's content.
        // fn content_source(content_source: &str)

        /// Prevents automatic link previews (snippets).
        ///
        /// `dont_parse_links`: If set to `1`, link snippets will not be created.
        fn dont_parse_links(dont_parse_links: bool)

        /// Disables notifications for mentions.
        ///
        /// `disable_mentions`: If set to `1`, mention notifications will be disabled.
        fn disable_mentions(disable_mentions: bool)

        /// Specifies the message intent.
        ///
        /// `intent`: A string describing the message intent.
        fn intent(intent: &str)

        /// Sets a subscription ID for future intents.
        ///
        /// `subscribe_id`: A positive number used for working with future intents.
        fn subscribe_id(subscribe_id: i32)
    }

    /// Attaches media to the message.
    ///
    /// `attachment`: Object or multiple objects attached to the message, such as photos, videos, or links.
    /// If multiple attachments, separate them with commas.
    /// Required if `message` is not set.
    pub fn attachment(mut self, media_type: &str, owner_id: i32, media_id: i64) -> Self {
        self.arg_fmt(
            "attachment",
            format_args!("{media_type}{owner_id}_{media_id}"),
        );
        self
    }

    /// Sets multiple recipient IDs.
    ///
    /// `ids`: A string of comma-separated user IDs. Max 100 recipients.
    /// Available only for community access tokens.
    pub fn peer_ids(mut self, ids: &[i64]) -> Self {
        self.remove_peer_id();
        self.arg("peer_ids", ids);
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

impl MethodBuilder<SendMessageEventAnswerMethod> {
    __method! {
        fn event_id(id: &str)
        fn user_id(id: i32)
        fn peer_id(id: i64)
    }

    pub fn event_data<T>(mut self, event: T) -> Self
    where
        T: serde::Serialize,
    {
        self.arg_json("event_data", event);
        self
    }
}

impl MethodBuilder<EditMessageMethod> {
    __method! {
        fn message(text: &str)
        fn lat(value: &str)
        fn peer_id(id: i64)
        fn long(value: &str)
        fn keep_forward_messages(forward: bool)
        fn keep_snippets(keep: bool)
        fn group_id(id: u32)
        fn dont_parse_links(parse: bool)
        fn disable_mentions(disable: bool)
        fn message_id(id: i32)
        fn conversation_message_id(id: i32)
    }

    pub fn attachment(mut self, media_type: &str, owner_id: i32, media_id: i64) -> Self {
        self.arg_fmt(
            "attachment",
            format_args!("{media_type}{owner_id}_{media_id}"),
        );
        self
    }
}

impl MethodBuilder<GetConversationMembersMethod> {
    __method! {
        fn offset(value: u16)
        fn count(value: u16)
        fn extended(extend: bool)
        fn fields(value: &str)
        fn group_id(id: u32)
    }
}
