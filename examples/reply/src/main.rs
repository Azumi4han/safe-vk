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
