use super::{Response, Service, ServiceExt};
use crate::RequestBuilder;
use futures_util::future::BoxFuture;
use std::sync::Arc;

pub struct BoxCloneService<T>(
    Box<dyn CloneService<T, Response = (), Future = BoxFuture<'static, Response<()>>> + Send>,
);

impl<T> BoxCloneService<T> {
    pub fn new<S>(inner: S) -> Self
    where
        S: Service<T, Response = ()> + Clone + Send + 'static,
        S::Future: Send + 'static,
    {
        let inner = inner.map_future(|f| Box::pin(f) as _);
        BoxCloneService(Box::new(inner))
    }
}

trait CloneService<R>: Service<R> {
    fn clone_box(
        &self,
    ) -> Box<dyn CloneService<R, Response = Self::Response, Future = Self::Future> + Send>;
}

impl<T> Service<T> for BoxCloneService<T> {
    type Response = ();
    type Future = BoxFuture<'static, Response<()>>;

    #[inline]
    fn poll_ready(&mut self, cx: &mut std::task::Context<'_>) -> std::task::Poll<Response<()>> {
        self.0.poll_ready(cx)
    }

    #[inline]
    fn call(&mut self, update: T, request: Arc<RequestBuilder>) -> Self::Future {
        self.0.call(update, request)
    }
}

impl<R, T> CloneService<R> for T
where
    T: Service<R> + Send + Clone + 'static,
{
    fn clone_box(
        &self,
    ) -> Box<dyn CloneService<R, Response = T::Response, Future = T::Future> + Send> {
        Box::new(self.clone())
    }
}

impl<T> Clone for BoxCloneService<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone_box())
    }
}
