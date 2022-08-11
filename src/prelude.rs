use async_trait::async_trait;
use digest::{Digest, Output};
use libp2p::swarm::NetworkBehaviour;

use crate::{
    core::{EntityBody, EpochHeader, VoterSet},
    message::Message,
    p2p, PeerKeys, Result,
};

pub trait P2PConfig: NetworkBehaviour + Sized {
    fn new(config: &p2p::config::Config, keys: &PeerKeys) -> Result<Self>;
}

pub trait NodeTypeConfig {
    type P2P: P2PConfig;
}

pub trait Entity<D: Digest> {
    fn to_body(&self) -> EntityBody;

    fn hash(&self) -> Output<D> {
        let body = self.to_body();

        body.hash::<D>()
    }
}

#[async_trait]
pub trait EpochPacker: Send + Sync + Clone + 'static {
    type Digest: Digest<OutputSize = digest::typenum::U32>;

    type Entity: Entity<Self::Digest>;

    async fn pack(&self) -> Result<(EpochHeader, Vec<Self::Entity>, VoterSet)>;
}

#[async_trait]
pub trait NetworkChannel {
    async fn recv_message(&self, message: Message) -> Result<()>;

    async fn send_message(&self) -> Result<Message>;
}
