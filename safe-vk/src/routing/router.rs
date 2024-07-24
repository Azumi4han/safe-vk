use std::{borrow::Cow, collections::HashMap, fmt, sync::Arc};

use super::{
    Handler, ListenerId, ListenerMethod, MethodEndpoint, RequestBuilder, RouteAdapter, RouteFuture,
    Update,
};
use crate::matchit;

pub(super) struct Listener<S> {
    listeners: HashMap<ListenerId, MethodListener<S>>,
    node: Arc<Node>,
    prev_listener_id: ListenerId,
}

#[must_use]
pub struct MethodListener<S = ()> {
    command: MethodEndpoint<S>,
    keyboard: MethodEndpoint<S>,
    any: MethodEndpoint<S>,
}

impl<S> Listener<S>
where
    S: Clone + Send + Sync + 'static,
{
    pub(super) fn listen(
        &mut self,
        listener: MethodListener<S>,
        method: ListenerMethod,
    ) -> Result<(), Cow<'static, str>> {
        let endpoint = if let Some((listener_id, method_listener)) = self
            .node
            .method_to_listener_id
            .get(&method)
            .and_then(|listener_id| {
                self.listeners
                    .get(listener_id)
                    .map(|svc| (*listener_id, svc))
            }) {
            let service = method_listener
                .clone()
                .merge_listeners(Some(&method), listener);
            self.listeners.insert(listener_id, service);

            return Ok(());
        } else {
            listener
        };

        let id = self.next_listener_id();
        self.set_node(method, id)?;
        self.listeners.insert(id, endpoint);
        Ok(())
    }

    fn set_node(&mut self, method: ListenerMethod, id: ListenerId) -> Result<(), String> {
        let mut node =
            Arc::try_unwrap(Arc::clone(&self.node)).unwrap_or_else(|node| (*node).clone());
        node.insert(method, id);
        self.node = Arc::new(node);
        Ok(())
    }

    pub(super) fn call_with_state(
        &self,
        update: Update,
        state: S,
        request: Arc<RequestBuilder>,
    ) -> RouteFuture {
        match self.node.at(&update) {
            Ok(id) => {
                let endpoint = self.listeners.get(&id).expect("no listener for id");
                endpoint.call_with_state(update, state, request)
            }
            Err(_) => RouteFuture::dummy(),
        }
    }

    pub(super) fn with_state<S2>(self, state: S) -> Listener<S2> {
        let listeners = self
            .listeners
            .into_iter()
            .map(|(id, endpoint)| {
                let endpoint: MethodListener<S2> = endpoint.with_state(state.clone());
                (id, endpoint)
            })
            .collect();
        Listener {
            listeners,
            node: self.node,
            prev_listener_id: self.prev_listener_id,
        }
    }

    fn next_listener_id(&mut self) -> ListenerId {
        let next_id = self
            .prev_listener_id
            .0
            .checked_add(1)
            .expect("Over `u32::MAX` listeners created");
        self.prev_listener_id = ListenerId(next_id);
        self.prev_listener_id
    }
}

impl<S> MethodListener<S>
where
    S: Clone,
{
    pub fn new() -> Self {
        Self {
            command: MethodEndpoint::None,
            keyboard: MethodEndpoint::None,
            any: MethodEndpoint::None,
        }
    }

    pub fn on<H, T>(self, handler: H) -> Self
    where
        H: Handler<T, S> + Sync,
        T: 'static,
        S: Send + Sync + 'static,
    {
        self.on_endpoint(MethodEndpoint::Listener(RouteAdapter::from_handler(
            handler,
        )))
    }

    fn on_endpoint(mut self, endpoint: MethodEndpoint<S>) -> Self {
        fn set_endpoint<S>(out: &mut MethodEndpoint<S>, endpoint: &MethodEndpoint<S>)
        where
            MethodEndpoint<S>: Clone,
            S: Clone,
        {
            *out = endpoint.clone();
        }
        set_endpoint(&mut self.command, &endpoint);
        set_endpoint(&mut self.any, &endpoint);
        self
    }

    pub fn with_state<S2>(self, state: S) -> MethodListener<S2> {
        MethodListener {
            command: self.command.with_state(&state),
            keyboard: self.keyboard.with_state(&state),
            any: self.any.with_state(&state),
        }
    }

    pub(crate) fn merge_listeners(
        mut self,
        method: Option<&ListenerMethod>,
        other: MethodListener<S>,
    ) -> Self {
        fn merge_inner<S>(
            method: Option<&ListenerMethod>,
            name: &str,
            first: MethodEndpoint<S>,
            second: MethodEndpoint<S>,
        ) -> MethodEndpoint<S> {
            match (first, second) {
                (MethodEndpoint::None, MethodEndpoint::None) => MethodEndpoint::None,
                (pick, MethodEndpoint::None) | (MethodEndpoint::None, pick) => pick,
                _ => {
                    if let Some(method) = method {
                        panic!(
                            "Handler for name `{name} with method: {method:?}` is already exists"
                        );
                    } else {
                        panic!("Cannot merge two same methods that both define {name}");
                    }
                }
            }
        }

        self.command = merge_inner(method, "COMMAND", self.command, other.command);
        self.keyboard = merge_inner(method, "KEYBOARD", self.keyboard, other.keyboard);
        self.any = merge_inner(method, "ANY", self.any, other.any);

        self
    }

    pub(crate) fn call_with_state(
        &self,
        update: Update,
        state: S,
        request: Arc<RequestBuilder>,
    ) -> RouteFuture {
        macro_rules! call {
            (
                $upd:expr,
                $svc:expr,
                $req:expr
            ) => {
                match $svc {
                    MethodEndpoint::None => {}
                    MethodEndpoint::Route(route) => {
                        return RouteFuture::new(route.clone().oneshot_inner($upd, $req));
                    }
                    MethodEndpoint::Listener(listener) => {
                        let listener = listener.clone().into_route(state);
                        return RouteFuture::new(listener.clone().oneshot_inner($upd, $req));
                    }
                }
            };
        }

        let Self {
            command,
            keyboard,
            any,
        } = self;

        call!(update, command, request);
        call!(update, keyboard, request);
        call!(update, any, request);

        RouteFuture::dummy()
    }
}

impl<S> Default for Listener<S> {
    fn default() -> Self {
        Self {
            listeners: Default::default(),
            node: Default::default(),
            prev_listener_id: ListenerId(0),
        }
    }
}

impl<S: 'static> Clone for Listener<S> {
    fn clone(&self) -> Self {
        Self {
            listeners: self.listeners.clone(),
            node: self.node.clone(),
            prev_listener_id: self.prev_listener_id,
        }
    }
}

impl<S> fmt::Debug for MethodListener<S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MethodListener")
            .field("command", &self.command)
            .field("any", &self.any)
            .finish()
    }
}

impl<S> fmt::Debug for Listener<S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Listener")
            .field("listeners", &self.listeners)
            .finish()
    }
}

impl<S: 'static> Clone for MethodListener<S> {
    fn clone(&self) -> Self {
        Self {
            command: self.command.clone(),
            keyboard: self.keyboard.clone(),
            any: self.any.clone(),
        }
    }
}

impl<S> Default for MethodListener<S>
where
    S: Clone,
{
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Default)]
struct Node {
    inner: HashMap<ListenerId, ListenerMethod>,
    listener_id_to_method: HashMap<ListenerId, Arc<ListenerMethod>>,
    method_to_listener_id: HashMap<Arc<ListenerMethod>, ListenerId>,
}

impl Node {
    pub(crate) fn insert(&mut self, method: ListenerMethod, val: ListenerId) {
        let method_arc = Arc::new(method.clone());
        self.inner.insert(val, method.clone());
        self.listener_id_to_method.insert(val, method_arc.clone());
        self.method_to_listener_id.insert(method_arc, val);
    }

    fn at(&self, event: &Update) -> Result<ListenerId, ()> {
        let command_listener = self
            .inner
            .values()
            .filter_map(|method| match method {
                ListenerMethod::Command {
                    update_type,
                    trigger,
                    filter,
                } => event.object.get("message").and_then(|msg| {
                    msg.get("text").and_then(|text| {
                        text.as_str().and_then(|message| {
                            if *update_type == event.update_type
                                && matchit(message, trigger, &filter)
                            {
                                self.method_to_listener_id.get(method).copied()
                            } else {
                                None
                            }
                        })
                    })
                }),
                //TODO: Make keyboard as route
                _ => None,
            })
            .next();

        if let Some(listener_id) = command_listener {
            Ok(listener_id)
        } else {
            self.inner
                .values()
                .find_map(|method| match method {
                    ListenerMethod::Watch => self.method_to_listener_id.get(method).copied(),
                    _ => None,
                })
                .ok_or(())
        }
    }
}
