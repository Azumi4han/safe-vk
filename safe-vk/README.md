![Static Badge](https://img.shields.io/badge/rust%20stable-1.78.0-orange)
![Static Badge](https://img.shields.io/badge/rust%20nightly-1.80.0-orange)
[![Crates.io](https://img.shields.io/crates/v/safe-vk)](https://crates.io/crates/safe-vk)

# safe-vk

Simple library with simple API for creating your own VK bot for conversations in **Rust 🦀**

## Current State

This library doesn't include all VK API methods. 
For now, it can handle simple tasks such as sending messages, 
getting users from a conversation, and sending photos.

## Future planning

- Improving asynchronous code
- Support more VK API methods
- More tests
- Documenting code
- Simplifying code 

## Prerequisites

Ensure you have Rust stable version **1.78.0** or **nightly version 1.80.0** installed. 
This library is tested and compatible with these versions.

## Overview

- Works with newest API version [**5.199**](https://dev.vk.com/en/reference/version/5.199)
- Utilizes `routes` same as in popular web frameworks
- has
  [serde_json](https://docs.rs/serde_json/1.0.111/serde_json/index.html),
  [tokio](https://docs.rs/tokio/1.35.1/tokio/index.html) and
  [reqwest](https://docs.rs/reqwest/0.11.23/reqwest/index.html) under the hood

## Installation

```bash
$ cargo add safe-vk
$ cargo update
```

## Greeting
```rust
use safe_vk::{extract::Ctx, responses::Message, Filter, Result, SafeVk};
use std::env;

async fn reply(update: Ctx<Message>) -> Result<()> {
    // Sending request to VK API
    update.message_text("hello from rust! 🦀").send().await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    let group_id: u32 = env::var("GROUP_ID")
        .unwrap_or_else(|_| "0".into())
        .parse()
        .expect("GROUP_ID must be a valid u32");

    let token = env::var("TOKEN").expect("TOKEN environment variable not set");

    // You can select Filter to change how strictly it listens to your command
    // You can find more info in the docs
    let bot = SafeVk::new().command("/hello", reply, Filter::Strict);

    // Start listening VK API
    safe_vk::start_polling(&token, group_id, bot).await.unwrap();
}
```

## More Examples

For more, try to play with [examples](examples). 
To run an example, use the following command in your terminal:

```shell
$ GROUP_ID=YOUR_GROUP_ID TOKEN=YOUR_TOKEN cargo run --example reply
$ GROUP_ID=YOUR_GROUP_ID TOKEN=YOUR_TOKEN cargo run --example comfyui
$ GROUP_ID=YOUR_GROUP_ID TOKEN=YOUR_TOKEN cargo run --example oobabooga
$ GROUP_ID=YOUR_GROUP_ID TOKEN=YOUR_TOKEN cargo run --example macros
$ GROUP_ID=YOUR_GROUP_ID TOKEN=YOUR_TOKEN cargo run --example keyboard
$ GROUP_ID=YOUR_GROUP_ID TOKEN=YOUR_TOKEN cargo run --example members
$ GROUP_ID=YOUR_GROUP_ID TOKEN=YOUR_TOKEN cargo run --example state
```

Don't forget to include your token and group ID!

## Motivation
My primary goal with this project is to learn how to work with asynchronous code 
and understand Rust's strong, safe type system. 
This project is **heavily** inspired by the [axum
crate](https://crates.io/crates/axum).
Feel free to contribute and propose new ideas!
**Updates will be made as I have motivation and free time.**

## License

`safe-vk` is available under the MIT license. See the [MIT License](LICENSE) file for more details
