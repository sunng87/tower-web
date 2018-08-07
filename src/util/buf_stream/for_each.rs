use super::BufStream;

use futures::{Future, IntoFuture, Async, Poll};

pub struct ForEach<S, F, U>
where
    U: IntoFuture
{
    stream: S,
    f: F,
    fut: Option<U::Future>,
}

impl<S, F, U> ForEach<S, F, U>
where
    U: IntoFuture,
{
    pub(crate) fn new(stream: S, f: F) -> Self {
        ForEach {
            stream,
            f,
            fut: None,
        }
    }
}

impl<S, F, U> Future for ForEach<S, F, U>
where
    S: BufStream,
    F: FnMut(S::Item) -> U,
    U: IntoFuture<Item = (), Error = S::Error>,
{
    type Item = ();
    type Error = S::Error;

    fn poll(&mut self) -> Poll<(), S::Error> {
        loop {
            if let Some(mut fut) = self.fut.take() {
                if !fut.poll()?.is_ready() {
                    self.fut = Some(fut);
                    return Ok(Async::NotReady);
                }
            }

            match try_ready!(self.stream.poll()) {
                Some(buf) => self.fut = Some((self.f)(buf).into_future()),
                None => return Ok(Async::Ready(())),
            }
        }
    }
}
