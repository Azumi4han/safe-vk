![Static Badge](https://img.shields.io/badge/rust%20stable-1.75.0-orange) 
![Static Badge](https://img.shields.io/badge/rust%20nightly-1.77.0-orange) 
# safe-vk

This is a simple library for creating your own VK bot in **Rust 🦀**

## Current State
 - Has only `reply` function to send a message into conversation
 - `command` function is for listening a command and it's in its 
 initial stage and will be enhanced
 - Only [Long Poll](https://dev.vk.com/en/api/bots-long-poll/getting-started)
is supported right now
 - Also has `watch` function that will be triggered each callback from VK
 - But these functions are enough to create a simple bot

## Overview
 - Works with API version [**5.199**](https://dev.vk.com/en/reference/version/5.199)
 - uses threads for more fast reply
 - has 
 [serde_json](https://docs.rs/serde_json/1.0.111/serde_json/index.html), 
 [tokio](https://docs.rs/tokio/1.35.1/tokio/index.html) and 
 [reqwest](https://docs.rs/reqwest/0.11.23/reqwest/index.html) under the hood
 - API is similar to [`axum`](https://docs.rs/axum/0.7.4/axum/index.html) 
 crate

## Installation
```bash
$ cargo add safe-vk
$ cargo update
```

## Example

```rust
use safe_vk::SafeVkBot;

const GROUP_ID: u32 = YOUR GROUP ID HERE;    
const TOKEN: &'static str = "YOUR TOKEN HERE"
   
// You can define your custom structure for custom requests
#[derive(Deserialize)]
pub struct ChatMembers {
    count: usize,
}

// Replying to command
async fn hello(ctx: Arc<Methods>) {
    ctx.reply("Hello from Rust!").await;
}

// Monitoring all incoming changes and do some funny stuff 
async fn changes(ctx: Arc<Methods>) {
    let context = ctx.context().await;
    for update in &context.updates {
        if let Some(message) = &update.object.message {
            println!("Message: {:?}", message.text);
            if message.text == "hello" {
                ctx.reply("Hello!").await;
            }
        }
    }
}

// Monitoring all incoming changes and also sends a custom request
async fn custom(ctx: Arc<Methods>) {
    let context = ctx.context().await;
    let ref request = ctx.request;

    if let Some(message) = &context.updates[0].object.message {
        let response = request
            .post(
                "https://api.vk.com/method",
                "messages.getConversationMembers",
                [("peer_id", message.peer_id.to_string())],
                {},
            )
            .await
            .unwrap();
        // Parsing response with macro for more clean code 
        let parsed = parse_response!(response, ChatMembers).unwrap();
        println!("Total members: {}", parsed.count);
    } else {
        ctx.reply("Failed to send a request!").await;
    }
}

#[tokio::test]
async fn main() {
    let bot = SafeVkBot::create(TOKEN);
        
    bot.command("!hello", hello)         // Listening a command
        .watch(changes)                  // Watching all changes   
        .watch(custom)                   // Watching all changes
        .start_polling(GROUP_ID)         // Starts a long poll server 
        .await;
}
```

## License
MIT
