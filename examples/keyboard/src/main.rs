use safe_vk::{
    auto_ok,
    extract::{Ctx, Update},
    parse_response,
    responses::{ButtonPressCallback, Message},
    Button, Filter, KeyboardColor, SafeVk, ShowSnackbar,
};
use serde::{Deserialize, Serialize};
use std::env;

const PAYLOAD: Payload = Payload { button: 1 };

#[derive(Serialize, Deserialize, PartialEq)]
pub struct Payload {
    button: u8,
}

#[auto_ok]
async fn keyboard(update: Ctx<Message>) {
    let button = Button::callback("Hello, world!", PAYLOAD, KeyboardColor::Primary);
    update
        .messages()
        .send()
        .random_id(0)
        .message("Press me!")
        .keyboard(false, true, &[[button]])?
        .await?;
}

#[auto_ok]
async fn changes(update: Ctx<Update>) {
    if let Ok(message) = parse_response!(update.object.clone(), ButtonPressCallback<Payload>) {
        if message.payload.button == 1 {
            update
                .messages()?
                .send_message_event_answer()
                .event_id(&message.event_id)
                .user_id(message.user_id)
                .event_data(ShowSnackbar::new("Thank you!"))
                .await?;
        }
    }
}

#[tokio::main]
async fn main() {
    let token = env::var("TOKEN").expect("TOKEN environment variable not set");

    let bot = SafeVk::new()
        .command("/keyboard", keyboard, Filter::Sensitive)
        .watch(changes);

    safe_vk::start_polling(&token, bot).await.unwrap();
}
