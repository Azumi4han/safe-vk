use safe_vk::{Method, Methods, SafeVkBot};
use std::time::{SystemTime, UNIX_EPOCH};
use std::{env, sync::Arc};

async fn help(ctx: Arc<Methods>) {
    ctx.reply(
        "Commands:\n`$hi`: will greet you\n`$number`: sends a random number between 0 and 10",
    )
    .await;
}

async fn hi(ctx: Arc<Methods>) {
    let context = ctx.context().await;
    let user_id = context.updates[0].object.message.as_ref().unwrap().from_id;
    let user = ctx.get_users(&[user_id]).await.unwrap();

    ctx.reply(&format!(
        "@id{}(Hello {} {}!)",
        user_id, user[0].first_name, user[0].last_name
    ))
    .await;
}

async fn random(ctx: Arc<Methods>) {
    let now = SystemTime::now();
    let since_the_epoch = now.duration_since(UNIX_EPOCH).expect("Time went backwards");
    let in_ms = since_the_epoch.as_millis();

    ctx.reply(&(in_ms % 11).to_string()).await
}

#[tokio::main]
async fn main() {
    let group_id: u32 = env::var("GROUP_ID")
        .unwrap_or_else(|_| "0".into())
        .parse()
        .expect("GROUP_ID must be a valid u32");

    let token = env::var("TOKEN").expect("TOKEN environment variable not set");

    let bot = SafeVkBot::create(&token);

    bot.command("$help", help)
        .command("$hi", hi)
        .command("$number", random)
        .start_polling(group_id)
        .await;
}
