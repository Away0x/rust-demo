use pin_project::pin_project;
use std::time::Duration;
use std::{
    fmt,
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};
use tokio::time::Sleep;
use tower::Service;

#[derive(Debug, Clone)]
struct Timeout<S> {
    inner: S,
    timeout: Duration,
}

impl<S> Timeout<S> {
    fn new(inner: S, timeout: Duration) -> Self {
        Timeout { inner, timeout }
    }
}

impl<S, Request> Service<Request> for Timeout<S>
where
    S: Service<Request>,
    S::Error: Into<BoxError>,
{
    type Response = S::Response;
    type Error = BoxError;
    type Future = ResponseFuture<S::Future>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx).map_err(Into::into)
    }

    fn call(&mut self, request: Request) -> Self::Future {
        let response_future = self.inner.call(request);
        let sleep = tokio::time::sleep(self.timeout);

        ResponseFuture {
            response_future,
            sleep,
        }
    }
}

#[pin_project]
struct ResponseFuture<F> {
    #[pin]
    response_future: F,
    #[pin]
    sleep: Sleep,
}

impl<F, Response, Error> Future for ResponseFuture<F>
where
    F: Future<Output = Result<Response, Error>>,
    Error: Into<BoxError>,
{
    type Output = Result<Response, BoxError>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();

        match this.response_future.poll(cx) {
            Poll::Ready(result) => {
                let result = result.map_err(Into::into);
                return Poll::Ready(result);
            }
            Poll::Pending => {}
        }

        match this.sleep.poll(cx) {
            Poll::Ready(()) => {
                let error = Box::new(TimeoutError(()));
                return Poll::Ready(Err(error));
            }
            Poll::Pending => {}
        }

        Poll::Pending
    }
}

#[derive(Debug, Default)]
struct TimeoutError(());

impl fmt::Display for TimeoutError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.pad("request timed out")
    }
}

impl std::error::Error for TimeoutError {}

type BoxError = Box<dyn std::error::Error + Send + Sync>;