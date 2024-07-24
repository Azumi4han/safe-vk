mod ctx;
mod keyboard;
mod state;

use crate::RequestBuilder;
use std::{future::Future, sync::Arc};

pub use self::{ctx::Ctx, keyboard::Keyboard, state::State};

pub type Update<T = serde_json::Value> = crate::responses::Event<T>;

pub trait FromUpdate<S>: Sized {
    fn from_update(
        update: Update,
        state: &S,
        request: Arc<RequestBuilder>,
    ) -> impl Future<Output = Result<Self, ()>> + Send;
}
