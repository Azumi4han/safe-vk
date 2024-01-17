use crate::Result;
use serde::Serialize;
use serde_json::Value;
use std::future::Future;

pub trait Request {
    fn new(access_token: String) -> Self;

    fn post<T: Serialize + Send, A: Serialize + Send + Sized>(
        &self,
        url: &str,
        method: &str,
        query: A,
        body: T,
    ) -> impl Future<Output = Result<Value>> + Send;

    fn get<T: Serialize + Send, A: Serialize + Send + Sized>(
        &self,
        url: &str,
        method: &str,
        query: A,
        body: T,
    ) -> impl Future<Output = Result<Value>> + Send;
}
