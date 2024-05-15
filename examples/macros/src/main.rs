use safe_vk::{auto_ok, extract::Ctx, responses::Message, Filter, SafeVk};
use std::env;

#[auto_ok]
async fn reply(update: Ctx<Message>) {
    update.message_text("hello from rust! 🦀").send().await?;
}

#[tokio::main]
async fn main() {
    let group_id: u32 = env::var("GROUP_ID")
        .unwrap_or_else(|_| "0".into())
        .parse()
        .expect("GROUP_ID must be a valid u32");

    let token = env::var("TOKEN").expect("TOKEN environment variable not set");

    let bot = SafeVk::new().command("/hello", reply, Filter::Strict);

    safe_vk::start_polling(&token, group_id, bot).await.unwrap();
}
