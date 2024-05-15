use safe_vk::{
    auto_ok,
    extract::{Ctx, Update},
    responses::Message,
    Button, Filter, KeyboardColor, SafeVk,
};
use serde::{Deserialize, Serialize};
use std::env;

const PAYLOAD: Payload = Payload { button: 1 };
const SNACKBAR: SnackBar = SnackBar {
    r#type: "show_snackbar",
    text: "hello?",
};

#[derive(Serialize, Deserialize, PartialEq)]
pub struct Payload {
    button: u8,
}

#[derive(Serialize)]
pub struct SnackBar {
    r#type: &'static str,
    text: &'static str,
}

#[auto_ok]
async fn keyboard(update: Ctx<Message>) {
    let button = Button::callback("Hello, world!", PAYLOAD, KeyboardColor::Primary);
    update
        .keyboard("Press me!", false, true, &[[button]])
        .await?;
}

#[auto_ok]
async fn changes(update: Ctx<Update>) {
    if let Ok(Some(object)) = update.keyboard_callback(SNACKBAR, PAYLOAD).await {
        if object.payload.button == 1 {
            update
                .message_text(format!("@id{}(Thank you!)", object.user_id))
                .send()
                .await?;
        }
    }
}

#[tokio::main]
async fn main() {
    let group_id: u32 = env::var("GROUP_ID")
        .unwrap_or_else(|_| "0".into())
        .parse()
        .expect("GROUP_ID must be a valid u32");

    let token = env::var("TOKEN").expect("TOKEN environment variable not set");

    let bot = SafeVk::new()
        .command("/keyboard", keyboard, Filter::Sensitive)
        .watch(changes);

    safe_vk::start_polling(&token, group_id, bot).await.unwrap();
}
