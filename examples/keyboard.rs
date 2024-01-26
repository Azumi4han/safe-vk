use safe_vk::{Button, ButtonAbstraction, KeyboardColor, Method, Methods, SafeVkBot};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

const GROUP_ID: u32 = YOUR_GROUP_ID_HERE;
const TOKEN: &'static str = "YOUR_TOKEN_HERE"
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

async fn alert(ctx: Arc<Methods>) {
    let button = Button::callback("Hello, world!", PAYLOAD, KeyboardColor::Primary);
    ctx.keyboard("Press me!", false, true, &[[button]])
        .await
        .expect("Unable to send keyboard");
    ctx.reply("a").await;
}

async fn changes(ctx: Arc<Methods>) {
    if let Ok(Some(payload)) = ctx.event_answer(SNACKBAR, PAYLOAD).await {
        if payload.button == 1 {
            ctx.reply("You clicked on me, thank you!").await;
        }
    }
}

#[tokio::main]
async fn main() {
    let bot = SafeVkBot::create(TOKEN);

    bot.watch(changes)
        .command("$alert", alert)
        .start_polling(GROUP_ID)
        .await;
}
