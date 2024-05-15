use safe_vk::{extract::Ctx, responses::Message, Filter, Result, SafeVk};
use std::env;

async fn members(update: Ctx<Message>) -> Result<()> {
    if let Ok(members) = update.message().get_members(None, Some(10), false).await {
        update
            .message_text(&format!("Total members: {}", members.count))
            .send()
            .await
            .unwrap();
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    let group_id: u32 = env::var("GROUP_ID")
        .unwrap_or_else(|_| "0".into())
        .parse()
        .expect("GROUP_ID must be a valid u32");

    let token = env::var("TOKEN").expect("TOKEN environment variable not set");

    let bot = SafeVk::new().command("/members", members, Filter::Sensitive);

    safe_vk::start_polling(&token, group_id, bot).await.unwrap();
}
