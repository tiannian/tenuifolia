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
pub trait Entity<ChainApp, MempoolApp> {
    async fn check(&self, chain: &ChainApp, mempool: &mut MempoolApp) -> Result<CheckReceipt>;
}

#[async_trait]
pub trait EpochPacker<ChainApp, MempoolApp> {
    async fn pack(&self, mempool: &mut Mempool<ChainApp, MempoolApp>) -> Result<EpochHeader>;
}
