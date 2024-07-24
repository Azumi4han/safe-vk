use std::{future::Future, pin::Pin, sync::Arc};

use crate::{
    extract::{FromUpdate, Update},
    service::HandlerService,
    RequestBuilder, Response,
};

pub trait Handler<T, S>: Clone + Send + Sized + 'static {
    type Future: Future<Output = Response<()>> + Send + 'static;

    fn call(self, update: Update, state: S, request: Arc<RequestBuilder>) -> Self::Future;

    fn with_state(self, state: S) -> HandlerService<Self, T, S> {
        HandlerService::new(self, state)
    }
}

impl<F, Fut, S> Handler<((),), S> for F
where
    F: FnOnce() -> Fut + Clone + Send + 'static,
    Fut: Future<Output = ()> + Send,
{
    type Future = Pin<Box<dyn Future<Output = Response<()>> + Send>>;

    fn call(self, _update: Update, _state: S, _request: Arc<RequestBuilder>) -> Self::Future {
        Box::pin(async move { Ok(self().await) })
    }
}

macro_rules! impl_handler {
    (
       [$($ty:ident),*]
    ) => {
        #[allow(non_snake_case, unused_mut)]
        impl<F, Fut, S, $($ty,)*> Handler<((), $($ty,)*), S> for F
        where
            F: FnOnce($($ty,)*) -> Fut + Clone + Send + 'static,
            Fut: Future<Output = Response<()>> + Send,
            S: Send + Sync + 'static,
            $( $ty: FromUpdate<S> + Send, )*
        {
            type Future = Pin<Box<dyn Future<Output = Response<()>> + Send>>;

            fn call(self, update: Update, state: S, request: Arc<RequestBuilder>) -> Self::Future {
                Box::pin(async move {
                    let state = &state;
                    let req = update;

                     $(
                        let $ty = $ty::from_update(req.clone(), state, request.clone()).await.unwrap();
                     )*


                    self($($ty,)*).await
                })
            }
        }
    };
}
all_the_tuples!(impl_handler);
