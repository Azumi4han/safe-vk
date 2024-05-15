use crate::{
    extract::Update, handler::Handler, routing::route::Route, RequestBuilder, Response, SafeVk,
};
use std::{
    future::Future,
    marker::PhantomData,
    pin::Pin,
    sync::Arc,
    task::{Context, Poll},
};

mod boxed_clone;
mod future;
mod map_future;

pub use boxed_clone::BoxCloneService;
pub use future::{Oneshot, RouteFuture};
pub use map_future::MapFuture;

pub trait Service<Callback> {
    type Response;
    type Future: Future<Output = Response<Self::Response>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Response<()>>;

    fn call(&mut self, update: Callback, request: Arc<RequestBuilder>) -> Self::Future;
}

impl<S, Callback> Service<Callback> for Box<S>
where
    S: Service<Callback> + ?Sized,
{
    type Response = S::Response;
    type Future = S::Future;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Response<()>> {
        (**self).poll_ready(cx)
    }

    fn call(&mut self, update: Callback, request: Arc<RequestBuilder>) -> Self::Future {
        (**self).call(update, request)
    }
}

impl<'a, S, Callback> Service<Callback> for &'a mut S
where
    S: Service<Callback> + 'a,
{
    type Response = S::Response;
    type Future = S::Future;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Response<()>> {
        (**self).poll_ready(cx)
    }

    fn call(&mut self, update: Callback, request: Arc<RequestBuilder>) -> Self::Future {
        (**self).call(update, request)
    }
}

impl Service<()> for SafeVk<()> {
    type Response = Self;
    type Future = std::future::Ready<Response<Self::Response>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Response<()>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, _update: (), _request: Arc<RequestBuilder>) -> Self::Future {
        std::future::ready(Ok(self.clone().with_state(())))
    }
}

impl Service<Update> for SafeVk<()> {
    type Response = ();
    type Future = RouteFuture;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Response<()>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, update: Update, request: Arc<RequestBuilder>) -> Self::Future {
        self.call_with_state(update, (), request)
    }
}

pub struct HandlerService<H, T, S> {
    handler: H,
    state: S,
    _marker: PhantomData<fn() -> T>,
}

impl<H, T, S> HandlerService<H, T, S> {
    pub(super) fn new(handler: H, state: S) -> Self {
        Self {
            handler,
            state,
            _marker: std::marker::PhantomData,
        }
    }
}

impl<H, T, S> Service<Update> for HandlerService<H, T, S>
where
    H: Handler<T, S> + Clone + Send + 'static,
    S: Clone + Send + Sync,
{
    type Response = ();
    type Future = Pin<Box<dyn Future<Output = Response<()>> + Send>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Response<()>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, update: Update, request: Arc<RequestBuilder>) -> Self::Future {
        let handler = self.handler.clone();
        let future = Handler::call(handler, update, self.state.clone(), request);

        Box::pin(async move { future.await.map(|_| ()) })
    }
}

impl<H, T, S> Clone for HandlerService<H, T, S>
where
    H: Clone,
    S: Clone,
{
    fn clone(&self) -> Self {
        Self {
            handler: self.handler.clone(),
            state: self.state.clone(),
            _marker: PhantomData,
        }
    }
}

impl Service<Update> for Route {
    type Response = ();
    type Future = RouteFuture;

    #[inline]
    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Response<()>> {
        Poll::Ready(Ok(()))
    }

    #[inline]
    fn call(&mut self, update: Update, request: Arc<RequestBuilder>) -> Self::Future {
        RouteFuture::new(self.oneshot_inner(update, request))
    }
}

pub trait ServiceExt<Callback>: Service<Callback> {
    fn oneshot(self, update: Callback, request: Arc<RequestBuilder>) -> Oneshot<Self, Callback>
    where
        Self: Sized,
    {
        Oneshot::new(self, update, request)
    }

    fn map_future<F, Fut, E>(self, f: F) -> MapFuture<Self, F>
    where
        Self: Sized,
        F: FnMut(Self::Future) -> Fut,
        Fut: Future<Output = Result<(), E>>,
    {
        MapFuture::new(self, f)
    }
}

impl<T: ?Sized, Callback> ServiceExt<Callback> for T where T: Service<Callback> {}
