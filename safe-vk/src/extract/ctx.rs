use super::{FromUpdate, RequestBuilder, Update};
use crate::{responses::Message, Error};
use serde_json::Value;
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

    pub(crate) fn find_peer_id(&self, object: &Value) -> crate::Result<i64> {
        match object {
            Value::Object(map) => {
                if let Some(peer_id) = map.get("peer_id").and_then(|id| id.as_i64()) {
                    return Ok(peer_id);
                }
                // Searching recursively only if peer_id is not found
                for v in map.values() {
                    if let Ok(id) = self.find_peer_id(v) {
                        return Ok(id);
                    }
                }
                Err(Error::PeerIdNotFound)
            }
            Value::Array(vec) => {
                for item in vec {
                    if let Ok(id) = self.find_peer_id(item) {
                        return Ok(id);
                    }
                }
                Err(Error::PeerIdNotFound)
            }
            _ => Err(Error::PeerIdNotFound),
        }
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
        let message: Message =
            serde_json::from_value(update.object).expect("Unable to parse an update");
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
