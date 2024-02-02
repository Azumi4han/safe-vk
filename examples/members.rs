use safe_vk::{Method, Methods, SafeVkBot};
use std::{env, sync::Arc};

async fn members(ctx: Arc<Methods>) {
    if let Ok(members) = ctx.get_members(None, Some(10), false).await {
        println!("{:#?}", members);
        ctx.reply(&format!("total members: {}", members.count))
            .await;
    }
}

#[tokio::main]
async fn main() {
    let group_id: u32 = env::var("GROUP_ID")
        .unwrap_or_else(|_| "0".into())
        .parse()
        .expect("GROUP_ID must be a valid u32");

    let token = env::var("TOKEN").expect("TOKEN environment variable not set");

    let bot = SafeVkBot::create(&token);

    bot.command("$members", members)
        .start_polling(group_id)
        .await;
}
