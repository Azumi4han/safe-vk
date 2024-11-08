use safe_vk::{auto_ok, extract::Ctx, responses::Message, Filter, SafeVk};
use std::env;

#[auto_ok]
async fn reply(update: Ctx<Message>) {
    update
        .messages()
        .send()
        .random_id(0)
        .message("hello from rust! 🦀")
        .await?;
}

#[tokio::main]
async fn main() {
    let token = env::var("TOKEN").expect("TOKEN environment variable not set");

    let bot = SafeVk::new().command("/hello", reply, Filter::Strict);

    safe_vk::start_polling(&token, bot).await.unwrap();
}
