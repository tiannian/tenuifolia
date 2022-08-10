use std::{
    pin::Pin,
    task::{Context, Poll},
};

use async_std::channel::{unbounded, Receiver, Sender};
use finality_grandpa::{Message, SignedMessage};
use libp2p::futures::Sink;

use crate::{core::EpochHash, Error, PeerSignature, Result};

pub struct ConsensusSide {
    pub grandpa_in: Receiver<Result<SignedMessage<EpochHash, u64, PeerSignature, Vec<u8>>>>,
    pub grandpa_out: Sender<Message<EpochHash, u64>>,
}

pub struct NetworkSide {
    pub grandpa_tx: Sender<Result<SignedMessage<EpochHash, u64, PeerSignature, Vec<u8>>>>,
    pub grandpa_rx: Receiver<Message<EpochHash, u64>>,
}

pub fn create_sides() -> (ConsensusSide, NetworkSide) {
    let (grandpa_tx, grandpa_in) = unbounded();

    let (grandpa_out, grandpa_rx) = unbounded();

    let cs = ConsensusSide {
        grandpa_in,
        grandpa_out,
    };

    let ns = NetworkSide {
        grandpa_tx,
        grandpa_rx,
    };

    (cs, ns)
}

#[derive(Debug, Clone)]
pub struct SinkSender<T> {
    pub(crate) sender: Sender<T>,
}

impl<T> From<Sender<T>> for SinkSender<T> {
    fn from(sender: Sender<T>) -> Self {
        Self { sender }
    }
}

impl<T> Sink<T> for SinkSender<T> {
    type Error = Error;

    fn poll_ready(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Result<()>> {
        Poll::Ready(Ok(()))
    }

    fn poll_flush(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Result<()>> {
        Poll::Ready(Ok(()))
    }

    fn poll_close(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Result<()>> {
        self.sender.close();
        Poll::Ready(Ok(()))
    }

    fn start_send(self: Pin<&mut Self>, item: T) -> Result<()> {
        self.sender
            .try_send(item)
            .map_err(|_| Error::TrySenderError)?;
        Ok(())
    }
}
