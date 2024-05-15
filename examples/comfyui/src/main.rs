use safe_vk::{
    auto_ok,
    extract::{Ctx, State},
    responses::Message,
    Filter, SafeVk,
};

use serde_json::{json, Value};
use std::{
    env,
    fs::File,
    sync::Arc,
    time::{SystemTime, UNIX_EPOCH},
};
use tokio::sync::Mutex;

const SERVER: &'static str = "127.0.0.1:8188";

#[derive(Clone)]
pub struct AppState {
    api: Value,
    cfg: f32,
    seed: u16,
    randomize: bool,
}

#[auto_ok]
async fn randomize(State(state): State<Arc<Mutex<AppState>>>, update: Ctx<Message>) {
    let mut state = state.lock().await;
    state.randomize = !state.randomize;

    update
        .message_text(format!("seed is randomized: {}", state.randomize))
        .send()
        .await?;
}

#[auto_ok]
async fn seed(State(state): State<Arc<Mutex<AppState>>>, update: Ctx<Message>) {
    let mut state = state.lock().await;
    let message = &update.message.text;

    if let Some(number_str) = message.strip_prefix("/seed ") {
        if let Ok(number) = number_str.parse::<u16>() {
            update
                .message_text(format!("New seed: {}", number))
                .send()
                .await?;
            state.seed = number;
            state.randomize = false;
        } else {
            panic!("The string after '/seed ' is not a valid u64 number.");
        }
    }
}

#[auto_ok]
async fn cfg(State(state): State<Arc<Mutex<AppState>>>, update: Ctx<Message>) {
    let mut state = state.lock().await;
    let message = &update.message.text;

    if let Some(number_str) = message.strip_prefix("/cfg ") {
        if let Ok(number) = number_str.parse::<f32>() {
            update
                .message_text(format!("changed from {} to {}", state.cfg, number))
                .send()
                .await?;
            state.cfg = number;
        } else {
            println!("The string after '/cfg ' is not a valid f32 number.");
        }
    }
}

#[auto_ok]
async fn imagine(State(state): State<Arc<Mutex<AppState>>>, update: Ctx<Message>) {
    let mut state = state.lock().await;
    let user = update.get_users(&[update.message.from_id]).await?;
    let seed = if state.randomize {
        random_seed()
    } else {
        state.seed
    };

    state.seed = seed;

    let client_id = format!("{:x}", seed);
    let prompt = update
        .message
        .text
        .split_once("/g")
        .map(|(_, after)| after)
        .unwrap();

    let default_prompt = "masterpiece, ((high quality, best shadow))";
    state.api["6"]["inputs"]["text"] = Value::from(format!("{},\n{}", default_prompt, prompt));
    state.api["3"]["inputs"]["seed"] = Value::from(seed);
    state.api["3"]["inputs"]["cfg"] = Value::from(state.cfg);

    let response = reqwest::Client::new()
        .post(format!("http://{SERVER}/prompt"))
        .json(&json!({ "prompt": state.api, "client_id": client_id }))
        .send()
        .await
        .unwrap()
        .json::<Value>()
        .await
        .unwrap();

    let prompt_id = response["prompt_id"].as_str().unwrap();
    let first_name = user[0].first_name.to_string();
    let last_name = user[0].last_name.to_string();

    update
        .message_text(format!(
            "{} {}\nID: {}\nCFG: {}\nSEED: {}",
            first_name,
            last_name,
            &prompt_id[..7],
            state.cfg,
            state.seed,
        ))
        .send()
        .await?;

    let response = get_history(prompt_id).await;

    let root = &response["outputs"]["9"]["images"][0];
    let image = get_image(
        root["filename"].as_str().unwrap(),
        root["subfolder"].as_str().unwrap(),
        root["type"].as_str().unwrap(),
    )
    .await;

    let photo = update.photos().upload(image, "image.png").await?;
    let owner_id = photo[0].owner_id;
    let photo_id = photo[0].id;

    update
        .message()
        .attachment("photo", owner_id, photo_id)
        .send()
        .await?;
}

#[auto_ok]
async fn help(update: Ctx<Message>) {
    update
        .message_text("/g --> Generates an image\n/rnd --> Randomizes the seed\n/cfg --> Sets custom cfg\n/seed --> Use your provided seed only")
        .send()
        .await?;
}

pub fn random_seed() -> u16 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_micros() as u16
}

async fn get_history(prompt_id: &str) -> Value {
    loop {
        let res = reqwest::Client::new()
            .get(format!("http://{SERVER}/history/{prompt_id}"))
            .send()
            .await
            .unwrap()
            .json::<Value>()
            .await
            .unwrap();

        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

        if res.get(prompt_id).is_some() {
            return res[prompt_id].clone();
        }
    }
}

async fn get_image(filename: &str, subfolder: &str, folder_type: &str) -> Vec<u8> {
    let query = [
        ("filename", filename),
        ("subfolder", subfolder),
        ("folder_type", folder_type),
    ];

    reqwest::Client::new()
        .get(format!("http://{SERVER}/view"))
        .query(&query)
        .send()
        .await
        .unwrap()
        .bytes()
        .await
        .unwrap()
        .to_vec()
}

#[tokio::main]
async fn main() {
    let group_id: u32 = env::var("GROUP_ID")
        .unwrap_or_else(|_| "0".into())
        .parse()
        .expect("GROUP_ID must be a valid u32");

    let token = env::var("TOKEN").expect("TOKEN environment variable not set");

    let file = File::open("examples/comfyui/workflow_api.json").unwrap();
    let reader = std::io::BufReader::new(file);
    let comfy_api: Value = serde_json::from_reader(reader).unwrap();

    let app_state = AppState {
        api: comfy_api,
        cfg: 8.0,
        seed: random_seed(),
        randomize: true,
    };

    let bot = SafeVk::new()
        .command("/help", help, Filter::Strict)
        .command("/g", imagine, Filter::Sensitive)
        .command("/rnd", randomize, Filter::Flexible)
        .command("/cfg", cfg, Filter::Sensitive)
        .command("/seed", seed, Filter::Sensitive)
        .with_state(Arc::new(Mutex::new(app_state)));

    safe_vk::start_polling(&token, group_id, bot).await.unwrap();
}
