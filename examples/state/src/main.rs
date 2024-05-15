use safe_vk::{
    extract::{Ctx, State},
    responses::Message,
    Filter, Result, SafeVk,
};
use std::env;

#[derive(Clone)]
pub struct AppState {
    version: &'static str,
}

async fn version(State(state): State<AppState>, update: Ctx<Message>) -> Result<()> {
    assert_eq!("1.0.0", state.version);
    update
        .message_text(format!("V{}", state.version))
        .send()
        .await
        .unwrap();

    Ok(())
}

#[tokio::main]
async fn main() {
    let group_id: u32 = env::var("GROUP_ID")
        .unwrap_or_else(|_| "0".into())
        .parse()
        .expect("GROUP_ID must be a valid u32");

    let token = env::var("TOKEN").expect("TOKEN environment variable not set");

    let bot = SafeVk::new()
        .command("/version", version, Filter::Sensitive)
        .with_state(AppState { version: "1.0.0" });

    safe_vk::start_polling(&token, group_id, bot).await.unwrap();
}
