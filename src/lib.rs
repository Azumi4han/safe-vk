mod engine;
mod error;
mod methods;
mod reqwest_ext;
mod responses;
mod storage;
mod traits;

pub use engine::*;
pub use error::*;
pub use methods::*;
pub use reqwest_ext::*;
pub use responses::*;
pub use storage::*;
pub use traits::*;

#[macro_export]
macro_rules! parse_response {
    ($value:expr, $type:ty) => {{
        use serde::de::Error;
        let parsed_result = match $value.get("response") {
            Some(response_value) => serde_json::from_value::<$type>(response_value.clone()),
            None => serde_json::from_value::<$type>($value.clone()),
        };
        parsed_result.map_err(|e| serde_json::Error::custom(e.to_string()))
    }};
}
