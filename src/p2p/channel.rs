use std::{
    pin::Pin,
    task::{Context, Poll},
};

use futures::{channel::mpsc::UnboundedSender, Sink};

use crate::{Error, Result};

#[derive(Debug, Clone)]
pub struct Sender<T> {
    pub(crate) sender: UnboundedSender<T>,
}

impl<T> Sink<T> for Sender<T> {
    type Error = Error;

    fn poll_ready(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<()>> {
        self.as_mut()
            .sender
            .poll_ready(cx)
            .map_err(Error::SendError)
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<()>> {
        Pin::new(&mut self.as_mut().sender)
            .poll_flush(cx)
            .map_err(Error::SendError)
    }

    fn poll_close(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<()>> {
        self.as_mut()
            .sender
            .poll_ready(cx)
            .map_err(Error::SendError)
    }

    fn start_send(self: Pin<&mut Self>, item: T) -> Result<()> {
        self.as_mut()
            .sender
            .start_send(item)
            .map_err(Error::SendError)
    }
}
