use super::{BoxCloneService, RequestBuilder, Response, Service, Update};
use futures_util::ready;
use pin_project_lite::pin_project;
use std::{future::Future, sync::Arc, task::Poll};

pin_project! {
    pub struct RouteFuture {
        #[pin]
        kind: RouteFutureKind,
    }
}

pin_project! {
    #[project = RouteFutureKindProj]
    pub enum RouteFutureKind {
        Future {
            #[pin]
            future: Oneshot<BoxCloneService<Update>, Update>,
        },
        DummyFuture,
    }
}

pin_project! {
    pub struct Oneshot<S: Service<Callback>, Callback> {
        #[pin]
        state: State<S, Callback>,
    }
}

pin_project! {
    #[project = StateProj]
    pub enum State<S: Service<Callback>, Callback> {
        NotReady {
            svc: S,
            upd: Option<Callback>,
            req: Option<Arc<RequestBuilder>>,
        },
        Called {
            #[pin]
            fut: S::Future,
        },
        Done,
    }
}

impl<S: Service<Callback>, Callback> State<S, Callback> {
    fn not_ready(svc: S, upd: Option<Callback>, req: Option<Arc<RequestBuilder>>) -> Self {
        Self::NotReady { svc, upd, req }
    }

    fn called(fut: S::Future) -> Self {
        Self::Called { fut }
    }
}

impl RouteFuture {
    pub(crate) fn new(future: Oneshot<BoxCloneService<Update>, Update>) -> Self {
        Self {
            kind: RouteFutureKind::Future { future },
        }
    }
    pub(crate) fn dummy() -> Self {
        Self {
            kind: RouteFutureKind::DummyFuture,
        }
    }
}

impl Future for RouteFuture {
    type Output = Response<()>;

    #[inline]
    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        let this = self.project();
        match this.kind.project() {
            RouteFutureKindProj::Future { future } => match future.poll(cx) {
                Poll::Ready(Ok(res)) => Poll::Ready(Ok(res)),
                Poll::Ready(Err(err)) => Poll::Ready(Err(err)),
                Poll::Pending => Poll::Pending,
            },
            RouteFutureKindProj::DummyFuture => Poll::Ready(Ok(())),
        }
    }
}

impl<S, Callback> Oneshot<S, Callback>
where
    S: Service<Callback>,
{
    pub fn new(svc: S, update: Callback, request: Arc<RequestBuilder>) -> Self {
        Oneshot {
            state: State::not_ready(svc, Some(update), Some(request)),
        }
    }
}

impl<S, Callback> Future for Oneshot<S, Callback>
where
    S: Service<Callback>,
{
    type Output = Response<S::Response>;

    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output> {
        let mut this = self.project();
        loop {
            match this.state.as_mut().project() {
                StateProj::NotReady { svc, upd, req } => {
                    let _ = ready!(svc.poll_ready(cx))?;
                    let f = svc.call(
                        upd.take().expect("already called"),
                        req.take().expect("already called"),
                    );
                    this.state.set(State::called(f));
                }
                StateProj::Called { fut } => {
                    let res = ready!(fut.poll(cx))?;
                    this.state.set(State::Done);
                    return Poll::Ready(Ok(res));
                }
                StateProj::Done => panic!("polled after complete"),
            }
        }
    }
}
