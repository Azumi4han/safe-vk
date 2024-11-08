![Static Badge](https://img.shields.io/badge/rust%20stable-1.79.0-orange)
![Static Badge](https://img.shields.io/badge/rust%20nightly-1.82.0-orange)
[![Crates.io](https://img.shields.io/crates/v/safe-vk)](https://crates.io/crates/safe-vk)

# safe-vk

Rust library for creating your own VK bot for conversations in **Rust 🦀**

## Current State

This library doesn’t include all VK API methods yet, but it has enough 
functionality to wrap around the [ComfyUI](https://github.com/comfyanonymous/ComfyUI) API
or even the [oobabooga](https://github.com/oobabooga/text-generation-webui)
API. See more in the [examples](examples).

## Future planning

- Improving asynchronous code
- Support more VK API methods
- Documenting code
- Simplifying code 
- Making a route for keyboards
- Add tests

## Prerequisites

Ensure you have Rust stable version **1.82.0** or **nightly version 1.84.0** installed. 
This library is tested and compatible with these versions.

## Overview

- Works with newest VK API version [**5.199**](https://dev.vk.com/en/reference/version/5.199)
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
    update.messages().send().random_id(0).message("hi").await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    let token = env::var("TOKEN").expect("TOKEN environment variable not set");

    let bot = SafeVk::new().command("/hello", reply, Filter::Strict);

    safe_vk::start_polling(&token, bot).await.unwrap();
}
```

## More Examples

For more, try to play with [examples](examples). 
To run an example, use the following command in your terminal:

```shell
$ TOKEN=YOUR_TOKEN cargo run --example reply
$ TOKEN=YOUR_TOKEN cargo run --example comfyui
$ TOKEN=YOUR_TOKEN cargo run --example oobabooga
$ TOKEN=YOUR_TOKEN cargo run --example macros
$ TOKEN=YOUR_TOKEN cargo run --example keyboard
$ TOKEN=YOUR_TOKEN cargo run --example members
$ TOKEN=YOUR_TOKEN cargo run --example state
```

Don't forget to include your token !

## Motivation
My primary goal with this project is to learn how to work with asynchronous code 
and understand Rust's strong, safe type system. 
This project is **heavily** inspired by the [axum
crate](https://crates.io/crates/axum).
Feel free to contribute and propose new ideas!
**Updates will be made as I have motivation and free time.**

## License

`safe-vk` is available under the MIT license. See the [MIT License](LICENSE) file for more details
