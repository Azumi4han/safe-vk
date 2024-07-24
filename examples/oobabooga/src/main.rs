use futures::StreamExt;
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
async fn update_message(update: &Ctx<Message>, message: &str, message_id: i32, peer_id: i64) {
    update
        .messages()
        .edit()
        .peer_id(peer_id)
        .conversation_message_id(message_id)
        .message(&message)
        .await?;
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

    if state.history.len() > 30 {
        state.history.clear();
    };

    let message_details = update
        .messages()
        .send()
        .random_id(0)
        .peer_ids(&[update.message.peer_id])
        .message("...")
        .await?
        .unwrap();

    let client = reqwest::Client::new();

    // Send a POST request to the SSE endpoint
    let response = client
        .post(OPEN_API)
        .json(&json!({
            "prompt": message,
            "messages": state.history,
            "stream": true,
            "max_tokens": 200,
            "temperature": 0.8,
            "top_p": 0.9,
            "top_k": 100,
            "top_a": 0,
            "mode": "instruct",
        }))
        .send()
        .await
        .unwrap();

    let mut stream = response.bytes_stream();
    let mut accumulated_message = String::new();
    let mut token_count = 0;

    while let Some(chunk) = stream.next().await {
        match chunk {
            Ok(bytes) => {
                let text = String::from_utf8_lossy(&bytes);
                if text.starts_with("data: ") {
                    let json_str = &text[6..]; // Strip "data: " prefix
                    if let Ok(json) = serde_json::from_str::<Value>(json_str) {
                        if let Some(content) = json["choices"][0]["delta"]["content"].as_str() {
                            let tokens: Vec<&str> = content.split_whitespace().collect();
                            token_count += tokens.len();
                            accumulated_message.push_str(content);

                            if token_count >= 5 {
                                update_message(
                                    &update,
                                    &accumulated_message,
                                    message_details[0].conversation_message_id,
                                    message_details[0].peer_id,
                                )
                                .await?;

                                token_count = 0;
                            }
                        }
                    } else {
                        eprintln!("Failed to parse chunk as JSON: {}", json_str);
                    }
                } else {
                    eprintln!("Unexpected chunk format: {}", text);
                }
            }
            Err(e) => eprintln!("Error while streaming response: {}", e),
        }
    }

    update_message(
        &update,
        &accumulated_message,
        message_details[0].conversation_message_id,
        message_details[0].peer_id,
    )
    .await?;

    println!("{:#?}", accumulated_message);
}

#[tokio::main]
async fn main() {
    let token = env::var("TOKEN").expect("TOKEN environment variable not set");

    let bot = SafeVk::new()
        .watch(answer)
        .with_state(Arc::new(Mutex::new(AppState {
            history: Vec::new(),
        })));

    safe_vk::start_polling(&token, bot).await.unwrap();
}
