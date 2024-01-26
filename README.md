![Static Badge](https://img.shields.io/badge/rust%20stable-1.75.0-orange)
![Static Badge](https://img.shields.io/badge/rust%20nightly-1.77.0-orange)
[![Crates.io](https://img.shields.io/crates/v/safe-vk)](https://crates.io/crates/safe-vk)

# safe-vk

This is a simple library for creating your own VK bot in **Rust 🦀**

## Current State

- Has some functions such as `reply` and `keyboard` for creating
  keyboard, see more in [examples](./examples/)
  initial stage and will be enhanced
- The `command` function, currently in its initial stage and is designed for handling user commands with plans for further enhancements
- Only [Long Poll](https://dev.vk.com/en/api/bots-long-poll/getting-started)
  is supported right now
- Also has `watch` function that will be triggered each callback from VK
- Provides a `get_users` function to fetch detailed information about users
- But these functions are enough to create a simple bot

## Prerequisites

Ensure you have Rust stable version 1.75.0 or nightly version 1.77.0 installed. This library is tested and compatible with these versions

## Overview

- Works with API version [**5.199**](https://dev.vk.com/en/reference/version/5.199)
- uses threads for more fast reply
- has
  [serde_json](https://docs.rs/serde_json/1.0.111/serde_json/index.html),
  [tokio](https://docs.rs/tokio/1.35.1/tokio/index.html) and
  [reqwest](https://docs.rs/reqwest/0.11.23/reqwest/index.html) under the hood

## Installation

```bash
$ cargo add safe-vk
$ cargo update
```

## Basic Usage

Here's a simple example to get you started with safe-vk:

```rust
use safe_vk::{SafeVkBot, Methods};
use std::sync::Arc;

const GROUP_ID: u32 = YOUR_GROUP_ID_HERE;
const TOKEN: &'static str = "YOUR_ACCESS_TOKEN_HERE";

async fn greet(ctx: Arc<Methods>) {
    ctx.reply("Hello from Rust!").await;
}

#[tokio::main]
async fn main() {
    let bot = SafeVkBot::create(TOKEN);

    bot.command("!greet", greet)
       .start_polling(GROUP_ID)
       .await;
}
```

## Advanced usage

```rust
use safe_vk::{Method, Methods, SafeVkBot};
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

const GROUP_ID: u32 = YOUR_GROUP_ID_HERE;
const TOKEN: &'static str = "YOUR_TOKEN_HERE";

async fn help(ctx: Arc<Methods>) {
    // Sends a message to conversation
    ctx.reply(
        "Commands:\n`$hi`: will greet you\n`$number`: sends a random number between 0 and 10",
    )
    .await;
}


async fn hi(ctx: Arc<Methods>) {
    let context = ctx.context().await;
    let user_id = context.updates[0].object.message.as_ref().unwrap().from_id;
    let user = ctx.get_users(&[user_id]).await.unwrap();

    // Mentions a user
    ctx.reply(&format!(
        "@id{}(Hello {} {}!)",
        user_id, user[0].first_name, user[0].last_name
    ))
    .await;
}

// Sends a random number between 0 and 10
async fn random(ctx: Arc<Methods>) {
    let now = SystemTime::now();
    let since_the_epoch = now.duration_since(UNIX_EPOCH).expect("Time went backwards");
    let in_ms = since_the_epoch.as_millis();

    ctx.reply(&(in_ms % 11).to_string()).await
}


#[tokio::main]
async fn main() {
    let bot = SafeVkBot::create(TOKEN);

    // Listening commands
    bot.command("$help", help)
        .command("$hi", hi)
        .command("$number", random)
        .start_polling(GROUP_ID) // Starts a polling server
        .await;
}
```

## Advanced Keyboard Interaction

Create interactive keyboards with callbacks:

```rust
const PAYLOAD: Payload = Payload { button: 1 };
const SNACKBAR: SnackBar = SnackBar {
    r#type: "show_snackbar",
    text: "Why...",
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

// Sends an alert message
async fn alert(ctx: Arc<Methods>) {
    let button = Button::callback("DO NOT!", PAYLOAD, KeyboardColor::Primary);
    ctx.keyboard("Don't press me", false, true, &[[button]])
        .await
        .expect("Unable to send keyboard");
}

async fn changes(ctx: Arc<Methods>) {
    if let Ok(Some(payload)) = ctx.event_answer(SNACKBAR, PAYLOAD).await {
        if payload.button == 1 {
            ctx.reply("I'm going to bite you!").await;
        }
    }
}
```

## Handling Multiple Buttons

Efficiently manage a keyboard with multiple buttons:

```rust
pub async fn many_buttons(ctx: Arc<Methods>) {
    let button_1 = Button::text("Button-1", PAYLOAD, KeyboardColor::Positive);
    let button_2 = Button::text("Button-2", PAYLOAD, KeyboardColor::Primary);
    let button_3 = Button::text("Button-3", PAYLOAD, KeyboardColor::Positive);
    let button_4 = Button::text("Button-4", PAYLOAD, KeyboardColor::Negative);
    let button_5 = Button::text("Button-5", PAYLOAD, KeyboardColor::Positive);
    let button_6 = Button::text("Button-6", PAYLOAD, KeyboardColor::Secondary);

    // The following code constructs a keyboard layout with two rows and three columns, as demonstrated below
    // This layout is part of the VK interface, designed to display a set of interactive buttons beneath a message
    // Note: The maximum keyboard layout supports up to 5 rows and 10 columns (5x10)
    //
    // Visual representation of the button arrangement:
    //
    //  ---------------------------------
    // | Press some button              |
    //  ---------------------------------
    // |  (btn-1)  (btn-2)  (btn-3)     |  <-- First row of buttons
    // |  (btn-4)  (btn-5)  (btn-6)     |  <-- Second row of buttons
    //  ---------------------------------

    ctx.keyboard(
        "Press some button",
        false, // Specifies whether the keyboard should be one-time
        true,  // Indicates whether the keyboard is inline
        &[
            [button_1, button_2, button_3], // First row of buttons
            [button_4, button_5, button_6], // Second row of buttons
        ],
    )
    .await
    .unwrap();
}
```

## Examples

For more, see [examples](./examples/). To run an examples, use the following commands in your terminal:

```shell
$ cargo run --example reply
$ cargo run --example keyboard
```

But don't forget to include your token and group ID

## License

`safe-vk` is available under the MIT license. See the [MIT License](LICENSE) file for more details
