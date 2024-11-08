use crate::{
    _define_abstraction,
    api::{
        EditMessageMethod, GetConversationMembersMethod, MethodBuilder, RequestBuilder,
        SendMessageEventAnswerMethod, SendMessageMethod, Write,
    },
};
use std::sync::Arc;

_define_abstraction! {
    AbstractionMessages for MethodBuilder {
        /// Sends a message
        fn send -> MethodBuilder<SendMessageMethod> {
            peer_id: true
        };

        fn edit -> MethodBuilder<EditMessageMethod> {
            peer_id: true
        };

        fn send_message_event_answer -> MethodBuilder<SendMessageEventAnswerMethod> {
            peer_id: true
        };

        fn get_conversation_members -> MethodBuilder<GetConversationMembersMethod> {
            peer_id: true
        };
    }
}
