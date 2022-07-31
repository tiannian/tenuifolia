use std::{future::Future, pin::Pin, task::{Context, Poll}};

use tokio::time::Sleep;

use crate::Error;

pub struct Timer {
    timer: Sleep,
}

impl Future for Timer {
    type Output = Result<(), Error>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let re = self.project().timer.poll(cx);

        match re {
            Poll::Ready(_) => Poll::Ready(Ok(())),
            Poll::Pending => Poll::Pending,
        }
    }
}

