use crate::{extract::Update, service::Service, RequestBuilder, Response};
use std::{
    future::{poll_fn, Future, IntoFuture},
    marker::PhantomData,
    pin::Pin,
    sync::Arc,
    task::{Context, Poll},
};

pub struct Polling<M, S> {
    request: Arc<RequestBuilder>,
    safevk: M,
    _marker: PhantomData<S>,
}

pub fn start_polling<M, S>(token: &str, group_id: u32, safevk: M) -> Polling<M, S>
where
    M: Service<(), Response = S>,
    S: Service<Update, Response = ()> + Clone + Send + 'static,
    S::Future: Send,
{
    let request = Arc::new(RequestBuilder::new(token, group_id));
    Polling {
        request,
        safevk,
        _marker: PhantomData,
    }
}

impl<M, S> IntoFuture for Polling<M, S>
where
    M: Service<(), Response = S> + Send + Clone + 'static + Service<Update>,
    <M as Service<Update>>::Future: Send,
    S: Service<Update, Response = ()> + Clone + Send + 'static,
    S::Future: Send,
{
    type Output = Response<()>;
    type IntoFuture = PollFuture;

    fn into_future(self) -> Self::IntoFuture {
        PollFuture(Box::pin(async move {
            let Self {
                request,
                mut safevk,
                _marker: _,
            } = self;
            loop {
                let res = request.build_long_poll_request().await?;

                if let Some(updates) = res.updates {
                    for event in updates {
                        poll_fn(|cx| <M as Service<Update>>::poll_ready(&mut safevk, cx))
                            .await
                            .unwrap();

                        let request = Arc::clone(&request);
                        let mut safevk = safevk.clone();

                        let _: tokio::task::JoinHandle<Response<()>> = tokio::spawn(async move {
                            match safevk.call(event, request).await {
                                Ok(..) => Ok(()),
                                Err(err) => panic!("{err}"),
                            }
                        });
                    }
                }
            }
        }))
    }
}

pub struct PollFuture(pub(super) futures_util::future::BoxFuture<'static, Response<()>>);

impl Future for PollFuture {
    type Output = Response<()>;

    #[inline]
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.0.as_mut().poll(cx)
    }
}
