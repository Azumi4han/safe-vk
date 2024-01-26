mod engine;
mod keyboard;
mod methods;
mod reqwest_ext;
mod responses;
mod traits;
mod util;

pub use engine::*;
pub use keyboard::*;
pub use methods::*;
pub use reqwest_ext::*;
pub use responses::*;
pub use traits::*;
pub use util::*;

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
