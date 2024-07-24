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
        .messages()
        .send()
        .random_id(0)
        .message(&format!("V{}", state.version))
        .await?;

    Ok(())
}

#[tokio::main]
async fn main() {
    let token = env::var("TOKEN").expect("TOKEN environment variable not set");

    let bot = SafeVk::new()
        .command("/version", version, Filter::Sensitive)
        .with_state(AppState { version: "1.0.0" });

    safe_vk::start_polling(&token, bot).await.unwrap();
}
