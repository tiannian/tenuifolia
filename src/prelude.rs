use async_trait::async_trait;
use libp2p::swarm::NetworkBehaviour;

use crate::{
    core::{CheckReceipt, EpochHeader},
    p2p, Mempool, PeerKeys, Result,
};

pub trait P2PConfig: NetworkBehaviour + Sized {
    fn new(config: &p2p::config::Config, keys: &PeerKeys) -> Result<Self>;
}

pub trait NodeTypeConfig {
    type P2P: P2PConfig;
}

#[async_trait]
pub trait Entity {
    type ChainApp;

    type MempoolApp;

    async fn check(
        &self,
        chain: &Self::ChainApp,
        mempool: &mut Self::MempoolApp,
    ) -> Result<CheckReceipt>;
}

#[async_trait]
pub trait EpochPacker<ChainApp, MempoolApp>: Send + Sync {
    type Digest: digest::Digest<OutputSize = digest::typenum::U32>;

    async fn pack(&self, mempool: &Mempool<ChainApp, MempoolApp>) -> Result<EpochHeader>;
}
