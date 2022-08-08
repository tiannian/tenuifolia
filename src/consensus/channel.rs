use std::sync::Arc;

use finality_grandpa::{SignedMessage, Message};
use futures::channel::mpsc::{UnboundedReceiver, UnboundedSender, unbounded};

use crate::{Error, PeerSignature, core::EpochHash, p2p::channel::Sender};

pub struct ConsensusSide {
    pub grandpa_in: Arc<UnboundedReceiver<Result<SignedMessage<EpochHash, u64, PeerSignature, Vec<u8>>, Error>>>,
    pub grandpa_out: Sender<Message<EpochHash, u64>>,
}

pub struct NetworkSide {
    pub grandpa_tx: UnboundedSender<Result<SignedMessage<EpochHash, u64, PeerSignature, Vec<u8>>, Error>>,
    pub grandpa_rx: Arc<UnboundedReceiver<Message<EpochHash, u64>>>,
}

pub fn create_sides() -> (ConsensusSide, NetworkSide) {
    let (grandpa_tx, grandpa_in) = unbounded();

    let (grandpa_out, grandpa_rx) = unbounded();

    let cs = ConsensusSide {
        grandpa_in: Arc::new(grandpa_in),
        grandpa_out: Sender { sender: grandpa_out },
    };

    let ns = NetworkSide {
        grandpa_tx,
        grandpa_rx: Arc::new(grandpa_rx),
    };

    (cs, ns)
}

