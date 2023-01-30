use std::error::Error;
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::sync::mpsc::{Receiver, UnboundedReceiver};

#[repr(transparent)]
pub struct Stream<R> {
    receiver: R,
}

impl <R> Stream<R> {
    pub fn new(receiver: R) -> Self {
        Stream { receiver }
    }
}

impl<T> futures::Stream for Stream<Receiver<T>> {
    type Item = T;
    //noinspection DuplicatedCode
    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<T>> {
        Pin::into_inner(self).receiver.poll_recv(cx)
    }
}

impl<T> futures::Stream for Stream<UnboundedReceiver<T>> {
    type Item = T;
    //noinspection DuplicatedCode
    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<T>> {
        Pin::into_inner(self).receiver.poll_recv(cx)
    }
}

/// The i2cdev error types produce lifetime errors when you try to return them.
/// This is a flaw of the crate's structure. This is a hack to work around it.
pub fn stringify_error<E: Error>(error: E) -> Box<dyn Error> {
    format!("{error}").into()
}
