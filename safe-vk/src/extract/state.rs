use super::{FromUpdate, RequestBuilder, Update};
use std::{
    ops::{Deref, DerefMut},
    sync::Arc,
};

#[derive(Debug, Default, Clone, Copy)]
pub struct State<S>(pub S);

pub trait FromRef<T> {
    fn from_ref(input: &T) -> Self;
}

impl<T> FromRef<T> for T
where
    T: Clone,
{
    fn from_ref(input: &T) -> Self {
        input.clone()
    }
}

impl<S> Deref for State<S> {
    type Target = S;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<S> DerefMut for State<S> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<OuterState, InnerState> FromUpdate<OuterState> for State<InnerState>
where
    InnerState: FromRef<OuterState>,
    OuterState: Send + Sync,
{
    async fn from_update(
        _update: Update,
        state: &OuterState,
        _request: Arc<RequestBuilder>,
    ) -> Result<Self, ()> {
        let inner_state = InnerState::from_ref(state);
        Ok(Self(inner_state))
    }
}
