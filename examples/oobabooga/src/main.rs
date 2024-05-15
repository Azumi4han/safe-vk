use safe_vk::{
    auto_ok,
    extract::{Ctx, State},
    responses::Message,
    SafeVk,
};
use serde_json::{json, Value};
use std::{env, sync::Arc};
use tokio::sync::Mutex;

const OPEN_API: &'static str = "http://127.0.0.1:5000/v1/chat/completions";

#[derive(Clone)]
pub struct AppState {
    history: Vec<Value>,
}

#[auto_ok]
async fn answer(State(state): State<Arc<Mutex<AppState>>>, update: Ctx<Message>) {
    let mut state = state.lock().await;
    if let Some(reply) = &update.message.reply_message {
        reply.from_id < 0
    } else {
        return Ok(());
    };

    let message = &update.message.text;
    let data = json!({"role": "user", "content": message});

    state.history.push(data);
    let response = reqwest::Client::new()
        .post(OPEN_API)
        .json(&json!({
            "prompt": message,
            "messages": state.history,
            "mode": "chat",  // Options: 'chat', 'chat-instruct', 'instruct'
            "instruction_template": "Alpaca",
        }))
        .send()
        .await
        .unwrap()
        .json::<Value>()
        .await
        .unwrap();

    let output = response["choices"][0]["message"]["content"]
        .as_str()
        .unwrap();

    state
        .history
        .push(json!({"role": "assistant", "content": output}));

    update.message_text(output).send().await?;
}

#[tokio::main]
async fn main() {
    let group_id: u32 = env::var("GROUP_ID")
        .unwrap_or_else(|_| "0".into())
        .parse()
        .expect("GROUP_ID must be a valid u32");

    let token = env::var("TOKEN").expect("TOKEN environment variable not set");

    let bot = SafeVk::new()
        .watch(answer)
        .with_state(Arc::new(Mutex::new(AppState {
            history: Vec::new(),
        })));

    safe_vk::start_polling(&token, group_id, bot).await.unwrap();
}
