use crate::{Ctx, LongPollResponse, RequestBuilder};
use std::future::Future;

pub trait Method {
    fn new(token: String) -> Self;

    // fn keyboard(&self, keys: &[&str]) -> impl Future<Output = ()> + Send;

    fn reply(&self, message: &str) -> impl Future<Output = ()> + Send;

    fn long_poll(&self, group_id: u32) -> impl Future<Output = LongPollResponse> + Send;

    fn connect(
        &self,
        server: &str,
        token: String,
        ts: String,
        wait: usize,
    ) -> impl Future<Output = Ctx> + Send;

    fn custom_request(&self) -> &RequestBuilder;

    fn context(&self) -> impl Future<Output = tokio::sync::RwLockReadGuard<'_, Ctx>> + Send;
}
