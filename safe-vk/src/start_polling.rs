use crate::{extract::Update, service::Service, Error, RequestBuilder, Response};
use std::{
    future::{poll_fn, Future, IntoFuture},
    marker::PhantomData,
    pin::Pin,
    sync::Arc,
    task::{Context, Poll},
};

pub struct Polling<M, S> {
    request: RequestBuilder,
    safevk: M,
    _marker: PhantomData<S>,
}

pub fn start_polling<M, S>(token: &str, safevk: M) -> Polling<M, S>
where
    M: Service<(), Response = S>,
    S: Service<Update, Response = ()> + Clone + Send + 'static,
    S::Future: Send,
{
    let request = RequestBuilder::new(token);
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

            let group_id = request.get_group_id().await?;
            let request = Arc::new(request);

            loop {
                match request.build_long_poll_request(group_id).await {
                    Ok(res) => {
                        if let Some(updates) = res.updates {
                            for event in updates {
                                poll_fn(|cx| <M as Service<Update>>::poll_ready(&mut safevk, cx))
                                    .await
                                    .unwrap();

                                let request_clone = Arc::clone(&request);
                                let mut safevk = safevk.clone();

                                let _: tokio::task::JoinHandle<Response<()>> =
                                    tokio::spawn(async move {
                                        match safevk.call(event, request_clone).await {
                                            Ok(..) => Ok(()),
                                            Err(err) => panic!("{err}"),
                                        }
                                    });
                            }
                        }
                    }
                    Err(Error::EventsOutdated { new_ts }) => request.update_ts(new_ts).await,
                    Err(Error::KeyExpired) => match request.get_long_poll_server(group_id).await {
                        Ok(new_session) => request.update_session(new_session).await,
                        Err(err) => {
                            eprintln!("Failed to fetch new long poll server session: {err}");
                        }
                    },
                    Err(Error::InformationLost) => {
                        match request.get_long_poll_server(group_id).await {
                            Ok(new_session) => {
                                request.update_ts(new_session.ts.clone()).await;
                                request.update_session(new_session).await;
                            }
                            Err(err) => {
                                eprintln!("Failed to fetch new long poll server session: {err}")
                            }
                        }
                    }
                    Err(err) => eprintln!("Error occured: {err}"),
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
