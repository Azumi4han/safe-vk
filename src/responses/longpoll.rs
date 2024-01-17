use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq)]
pub struct LongPollResponse {
    pub key: String,
    pub server: String,
    pub ts: String,
}
