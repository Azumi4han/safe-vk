use super::{Handler, RequestBuilder, Route, RouteFuture, SafeVk, Update};
use crate::service::Service;

use std::sync::{Arc, Mutex};

pub struct RouteAdapter<S>(Mutex<Box<dyn ErasedIntoRoute<S>>>);

impl<S> RouteAdapter<S>
where
    S: Clone + Send + Sync + 'static,
{
    pub(crate) fn from_handler<H, T>(handler: H) -> Self
    where
        H: Handler<T, S>,
        T: 'static,
    {
        Self(Mutex::new(Box::new(MakeErasedHandler {
            handler,
            into_route: |handler, state| Route::new(Handler::with_state(handler, state)),
        })))
    }
}

impl<S> RouteAdapter<S> {
    pub(crate) fn into_route(self, state: S) -> Route {
        self.0.into_inner().unwrap().into_route(state)
    }
}

pub(crate) trait ErasedIntoRoute<S>: Send {
    fn clone_box(&self) -> Box<dyn ErasedIntoRoute<S>>;

    fn into_route(self: Box<Self>, state: S) -> Route;

    #[allow(unused)]
    fn call_with_state(
        self: Box<Self>,
        update: Update,
        request: Arc<RequestBuilder>,
        state: S,
    ) -> RouteFuture;
}

pub(crate) struct MakeErasedHandler<H, S> {
    pub(crate) handler: H,
    pub(crate) into_route: fn(H, S) -> Route,
}

impl<H, S> ErasedIntoRoute<S> for MakeErasedHandler<H, S>
where
    H: Clone + Send + 'static,
    S: 'static,
{
    fn clone_box(&self) -> Box<dyn ErasedIntoRoute<S>> {
        Box::new(self.clone())
    }

    fn into_route(self: Box<Self>, state: S) -> Route {
        (self.into_route)(self.handler, state)
    }

    fn call_with_state(
        self: Box<Self>,
        update: Update,
        request: Arc<RequestBuilder>,
        state: S,
    ) -> RouteFuture {
        self.into_route(state).call(update, request)
    }
}

pub(crate) struct MakeErasedRouter<S> {
    pub(crate) router: SafeVk<S>,
    pub(crate) into_route: fn(SafeVk<S>, S) -> Route,
}

impl<S> ErasedIntoRoute<S> for MakeErasedRouter<S>
where
    S: Clone + Send + Sync + 'static,
{
    fn clone_box(&self) -> Box<dyn ErasedIntoRoute<S>> {
        Box::new(self.clone())
    }

    fn into_route(self: Box<Self>, state: S) -> Route {
        (self.into_route)(self.router, state)
    }

    fn call_with_state(
        self: Box<Self>,
        update: Update,
        request: Arc<RequestBuilder>,
        state: S,
    ) -> RouteFuture {
        self.router.call_with_state(update, state, request)
    }
}

impl<H, S> Clone for MakeErasedHandler<H, S>
where
    H: Clone,
{
    fn clone(&self) -> Self {
        Self {
            handler: self.handler.clone(),
            into_route: self.into_route,
        }
    }
}

impl<S> Clone for MakeErasedRouter<S>
where
    S: Clone,
{
    fn clone(&self) -> Self {
        Self {
            router: self.router.clone(),
            into_route: self.into_route,
        }
    }
}

impl<S> Clone for RouteAdapter<S> {
    fn clone(&self) -> Self {
        Self(Mutex::new(self.0.lock().unwrap().clone_box()))
    }
}
