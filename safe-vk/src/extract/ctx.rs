use super::{FromUpdate, RequestBuilder, Update};
use crate::responses::Message;
use std::sync::Arc;

#[derive(Clone)]
pub struct Ctx<T> {
    pub request: Arc<RequestBuilder>,
    body: T,
}

impl<T> Ctx<T> {
    pub fn request(&self) -> &RequestBuilder {
        &self.request
    }

    pub fn new(request: Arc<RequestBuilder>, body: T) -> Ctx<T> {
        Ctx { request, body }
    }
}

impl<T> std::ops::Deref for Ctx<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.body
    }
}

impl<S> FromUpdate<S> for Ctx<Message>
where
    S: Send + Sync,
{
    async fn from_update(
        update: Update,
        _state: &S,
        request: Arc<RequestBuilder>,
    ) -> Result<Self, ()> {
        let message: Message = serde_json::from_value(update.object).unwrap();
        Ok(Ctx {
            request,
            body: message,
        })
    }
}

impl<S> FromUpdate<S> for Ctx<Update>
where
    S: Send + Sync,
{
    async fn from_update(
        update: Update,
        _state: &S,
        request: Arc<RequestBuilder>,
    ) -> Result<Self, ()> {
        Ok(Ctx {
            request,
            body: update,
        })
    }
}
