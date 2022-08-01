use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

use crate::{core::EpochHash, Result};

pub struct BestChain {
    future: Pin<Box<dyn Future<Output = Result<Option<(EpochHash, u64)>>> + Send>>,
}

impl BestChain {
    pub fn new<F: Future<Output = Result<Option<(EpochHash, u64)>>> + Send>(f: F) -> Self {
        Self {
            future: Box::pin(f),
        }
    }
}

impl Future for BestChain {
    type Output = Result<Option<(EpochHash, u64)>>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.future.as_mut().poll(cx)
    }
}
