use crate::{Ctx, LongPollResponse, Members, RequestBuilder, Result, User};
use std::future::Future;

pub trait Method {
    fn new(token: String) -> Self;

    fn keyboard<T: serde::Serialize + Send, N: crate::NdArray<T> + Send>(
        &self,
        message: &str,
        one_time: bool,
        inline: bool,
        buttons: N,
    ) -> impl Future<Output = Result<()>> + Send;

    fn event_answer<
        'de,
        T: serde::Serialize + Send,
        A: serde::de::DeserializeOwned + PartialEq + serde::Serialize + Send,
    >(
        &self,
        event_data: T,
        payload: A,
    ) -> impl Future<Output = Result<Option<A>>> + Send;

    fn reply(&self, message: &str) -> impl Future<Output = ()> + Send;

    fn long_poll(&self, group_id: u32) -> impl Future<Output = LongPollResponse> + Send;

    fn connect(
        &self,
        server: &str,
        token: String,
        ts: String,
        wait: usize,
    ) -> impl Future<Output = Ctx> + Send;

    fn get_users(&self, user_ids: &[i32]) -> impl Future<Output = Result<Vec<User>>> + Send;

    fn get_members(
        &self,
        offset: Option<u16>,
        count: Option<u16>,
        extended: bool,
    ) -> impl Future<Output = Result<Members>> + Send;

    fn custom_request(&self) -> &RequestBuilder;

    fn context(&self) -> impl Future<Output = tokio::sync::RwLockReadGuard<'_, Ctx>> + Send;
}
