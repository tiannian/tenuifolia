use async_trait::async_trait;
use libp2p::swarm::NetworkBehaviour;

use crate::{core::EpochHeader, message::Message, p2p, PeerKeys, Result};

pub trait P2PConfig: NetworkBehaviour + Sized {
    fn new(config: &p2p::config::Config, keys: &PeerKeys) -> Result<Self>;
}

pub trait NodeTypeConfig {
    type P2P: P2PConfig;
}

#[async_trait]
pub trait EpochPacker: Send + Sync + Clone + 'static {
    type Digest: digest::Digest<OutputSize = digest::typenum::U32>;

    async fn pack(&self) -> Result<EpochHeader>;
}

#[async_trait]
pub trait NetworkChannel {
    async fn recv_message(&self, message: Message) -> Result<()>;

    async fn send_message(&self) -> Result<Message>;
}
