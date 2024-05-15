use crate::Error;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ButtonPressCallback<T> {
    /// The ID of the conversation message associated with the button press.
    pub conversation_message_id: i32,
    /// A unique identifier for this particular event occurrence.
    pub event_id: String,
    /// An object containing additional data sent with the button press.
    /// The structure of this object is defined by the button's payload.
    pub payload: T,
    /// The ID of the conversation (chat) where the button was pressed.
    pub peer_id: i32,
    /// The user ID of the user who pressed the button.
    pub user_id: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EventAnswer(i8);

impl EventAnswer {
    pub fn get_status(&self) -> crate::Result<i8> {
        let status = self.0;
        match status {
            1 => Ok(status),
            _ => Err(Error::EventAnswerUnkownStatus { status }),
        }
    }
}
