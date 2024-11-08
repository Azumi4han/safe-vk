use crate::{parse_response, responses::events::User, Method};
use serde::Deserialize;
use std::future::IntoFuture;

#[derive(Deserialize, Method, Debug)]
#[method_path("users.get")]
pub struct GetUsers(pub Vec<User>);
