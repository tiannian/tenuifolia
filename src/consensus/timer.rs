use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
    time::Duration,
};

use async_std::task::sleep;

use crate::Error;

pub struct Timer {
    timer: Pin<Box<dyn Future<Output = ()>>>,
}

impl Timer {
    pub fn sleep(d: Duration) -> Self {
        Timer {
            timer: Box::pin(sleep(d)),
        }
    }
}

impl Future for Timer {
    type Output = Result<(), Error>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let re = self.timer.as_mut().poll(cx);

        match re {
            Poll::Ready(_) => Poll::Ready(Ok(())),
            Poll::Pending => Poll::Pending,
        }
    }
}
