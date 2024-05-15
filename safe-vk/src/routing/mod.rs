use std::{fmt, sync::Arc};

pub mod adapter;
pub mod route;
pub mod route_method;
pub mod router;

use self::{
    adapter::RouteAdapter,
    route::Route,
    route_method::ListenerMethod,
    router::{Listener, MethodListener},
};

use super::{extract::Update, service::RouteFuture, Filter};
use crate::{handler::Handler, RequestBuilder};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct ListenerId(u32);

pub struct SafeVk<S = ()> {
    inner: Arc<SafeVkInner<S>>,
}

pub struct SafeVkInner<S> {
    method_listener: Listener<S>,
}

impl<S> Clone for SafeVk<S> {
    fn clone(&self) -> Self {
        Self {
            inner: Arc::clone(&self.inner),
        }
    }
}

impl<S> Default for SafeVk<S>
where
    S: Clone + Send + Sync + 'static,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<S> fmt::Debug for SafeVk<S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Safevk")
            .field("method_listener", &self.inner.method_listener)
            .finish()
    }
}

impl<S> SafeVk<S>
where
    S: Clone + Send + Sync + 'static,
{
    pub fn new() -> Self {
        Self {
            inner: Arc::new(SafeVkInner {
                method_listener: Default::default(),
            }),
        }
    }

    fn map_inner<F, S2>(self, f: F) -> SafeVk<S2>
    where
        F: FnOnce(SafeVkInner<S>) -> SafeVkInner<S2>,
    {
        SafeVk {
            inner: Arc::new(f(self.into_inner())),
        }
    }

    fn tap_inner_mut<F>(self, f: F) -> Self
    where
        F: FnOnce(&mut SafeVkInner<S>),
    {
        let mut inner = self.into_inner();
        f(&mut inner);
        SafeVk {
            inner: Arc::new(inner),
        }
    }

    fn into_inner(self) -> SafeVkInner<S> {
        match Arc::try_unwrap(self.inner) {
            Ok(inner) => inner,
            Err(arc) => SafeVkInner {
                method_listener: arc.method_listener.clone(),
            },
        }
    }

    pub fn command<H, T>(self, trigger: impl Into<String>, handler: H, filter: Filter) -> Self
    where
        H: Handler<T, S> + Sync,
        T: 'static,
    {
        self.tap_inner_mut(|this| {
            this.method_listener
                .listen(
                    MethodListener::new().on(handler),
                    ListenerMethod::command(trigger.into(), filter),
                )
                .unwrap()
        })
    }

    pub fn watch<H, T>(self, handler: H) -> Self
    where
        H: Handler<T, S> + Sync,
        T: 'static,
    {
        self.tap_inner_mut(|this| {
            this.method_listener
                .listen(MethodListener::new().on(handler), ListenerMethod::Watch)
                .unwrap()
        })
    }

    pub(crate) fn call_with_state(
        &self,
        update: Update,
        state: S,
        request: Arc<RequestBuilder>,
    ) -> RouteFuture {
        self.inner
            .method_listener
            .call_with_state(update, state, request)
    }

    pub fn with_state<S2>(self, state: S) -> SafeVk<S2> {
        self.map_inner(|this| SafeVkInner {
            method_listener: this.method_listener.with_state(state.clone()),
        })
    }
}

pub enum MethodEndpoint<S = ()> {
    None,
    Listener(RouteAdapter<S>),
    Route(Route),
}

impl<S> MethodEndpoint<S>
where
    S: Clone,
{
    fn with_state<S2>(self, state: &S) -> MethodEndpoint<S2> {
        match self {
            MethodEndpoint::None => MethodEndpoint::None,
            MethodEndpoint::Listener(handler) => {
                MethodEndpoint::Route(handler.into_route(state.clone()))
            }
            MethodEndpoint::Route(route) => MethodEndpoint::Route(route),
        }
    }
}

impl<S> Clone for MethodEndpoint<S> {
    fn clone(&self) -> Self {
        match self {
            Self::None => Self::None,
            Self::Listener(inner) => Self::Listener(inner.clone()),
            Self::Route(inner) => Self::Route(inner.clone()),
        }
    }
}

impl<S> fmt::Debug for MethodEndpoint<S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::None => f.debug_tuple("None").finish(),
            Self::Route(_) => f.debug_tuple("Route").finish(),
            Self::Listener(_) => f.debug_tuple("Listener").finish(),
        }
    }
}
