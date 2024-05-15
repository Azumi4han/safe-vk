use crate::{
    extract::Update,
    service::{BoxCloneService, Oneshot, Service, ServiceExt},
    RequestBuilder,
};
use std::{
    fmt,
    sync::{Arc, Mutex},
};

pub struct Route(Mutex<BoxCloneService<Update>>);

impl Route {
    pub(crate) fn new<T>(svc: T) -> Self
    where
        T: Service<Update, Response = ()> + Clone + Send + 'static,
        T::Response: 'static,
        T::Future: Send + 'static,
    {
        Self(Mutex::new(BoxCloneService::new(svc)))
    }

    pub(crate) fn oneshot_inner(
        &mut self,
        update: Update,
        request: Arc<RequestBuilder>,
    ) -> Oneshot<BoxCloneService<Update>, Update> {
        self.0.get_mut().unwrap().clone().oneshot(update, request)
    }
}

impl Clone for Route {
    fn clone(&self) -> Self {
        Self(Mutex::new(self.0.lock().unwrap().clone()))
    }
}

impl fmt::Debug for Route {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Route").finish()
    }
}
