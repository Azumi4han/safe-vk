use super::{FromUpdate, RequestBuilder, Update};
use crate::responses::ButtonPressCallback;
use serde::de::DeserializeOwned;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct Keyboard<T>(pub ButtonPressCallback<T>);

impl<T> std::ops::Deref for Keyboard<T> {
    type Target = ButtonPressCallback<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T, S> FromUpdate<S> for Keyboard<T>
where
    T: DeserializeOwned,
    S: Send + Sync,
{
    async fn from_update(
        update: Update,
        _state: &S,
        _request: Arc<RequestBuilder>,
    ) -> Result<Self, ()> {
        let callback: ButtonPressCallback<T> =
            serde_json::from_value(update.object).expect("Unable to parse a button callback");

        Ok(Self(callback))
    }
}
