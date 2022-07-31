use libp2p::swarm::NetworkBehaviour;

use crate::{core::BlockHash, p2p, PeerKeys, Result};

pub trait P2PConfig: NetworkBehaviour + Sized {
    fn new(config: &p2p::config::Config, keys: &PeerKeys) -> Result<Self>;
}

pub trait NodeTypeConfig {
    type P2P: P2PConfig;
}

pub trait Store {
    fn get_blockhash_by_height(&self, height: u64) -> Result<BlockHash>;

    fn get_blockhash_by_height_batch(&self, begin: u64, end: u64) -> Result<Vec<BlockHash>>;

    fn get_blockhash_sequence(&self, begin: BlockHash, end: BlockHash) -> Result<Vec<BlockHash>>;
}
