use super::Service;
use crate::Response;
use std::{
    future::Future,
    task::{Context, Poll},
};

#[derive(Clone)]
pub struct MapFuture<S, F> {
    inner: S,
    f: F,
}

impl<S, F> MapFuture<S, F> {
    pub fn new(inner: S, f: F) -> Self {
        Self { inner, f }
    }
}

impl<R, S, F, T, Fut> Service<R> for MapFuture<S, F>
where
    S: Service<R>,
    F: FnMut(S::Future) -> Fut,
    Fut: Future<Output = Response<T>>,
{
    type Response = T;
    type Future = Fut;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Response<()>> {
        self.inner.poll_ready(cx).map_err(From::from)
    }

    fn call(&mut self, update: R, request: std::sync::Arc<crate::RequestBuilder>) -> Self::Future {
        (self.f)(self.inner.call(update, request))
    }
}
