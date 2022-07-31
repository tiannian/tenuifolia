use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
    time::Duration,
};

use tokio::time::{sleep, Sleep};

use crate::Error;

pub struct Timer {
    timer: Pin<Box<Sleep>>,
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

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let re = self.timer.as_mut().poll(cx);

        match re {
            Poll::Ready(_) => Poll::Ready(Ok(())),
            Poll::Pending => Poll::Pending,
        }
    }
}
