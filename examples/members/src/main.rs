use safe_vk::{auto_ok, extract::Ctx, responses::Message, Filter, SafeVk};
use std::env;

#[auto_ok]
async fn members(update: Ctx<Message>) {
    let members = update.users().get_conversation_members().await?;

    update
        .messages()
        .send()
        .random_id(0)
        .message(&format!("Total members: {}", members.count))
        .await?;
}

#[tokio::main]
async fn main() {
    let token = env::var("TOKEN").expect("TOKEN environment variable not set");

    let bot = SafeVk::new().command("/members", members, Filter::Sensitive);

    safe_vk::start_polling(&token, bot).await.unwrap();
}
