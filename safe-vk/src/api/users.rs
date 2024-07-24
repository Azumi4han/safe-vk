use super::Write;
use crate::{
    api_func, chained_method_fn,
    extract::{Ctx, Update},
    parse_response,
    responses::{events::User, Message},
    RequestBuilder, Result, VK,
};
use std::{fmt, future::IntoFuture, sync::Arc};

pub struct UsersBuilder {
    request: Arc<RequestBuilder>,
    _peer_id: i64,
}

chained_method_fn!(
    GetUsers,
    Vec<User>,
    "users.get",
    user_ids(&[i32]),
    fields(&str),
    name_case(&str),
    from_group_id(u32)
);

impl UsersBuilder {
    pub fn new(request: Arc<RequestBuilder>, _peer_id: i64) -> UsersBuilder {
        UsersBuilder { request, _peer_id }
    }

    pub fn get(&self) -> GetUsers {
        let request = Arc::clone(&self.request);
        GetUsers::new(request, None)
    }
}

impl Ctx<Message> {
    pub fn users(&self) -> UsersBuilder {
        let peer_id = self.message.peer_id;
        UsersBuilder::new(self.request.clone(), peer_id)
    }
}

impl Ctx<Update> {
    pub fn users(&self) -> Result<UsersBuilder> {
        let peer_id = self.find_peer_id(&self.object)?;
        Ok(UsersBuilder::new(self.request.clone(), peer_id))
    }
}
