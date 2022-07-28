use libp2p::swarm::NetworkBehaviour;

use crate::{p2p, PeerKeys, Result};

pub trait P2PConfig: NetworkBehaviour + Sized {
    fn new(config: &p2p::config::Config, keys: &PeerKeys) -> Result<Self>;
}

pub trait NodeTypeConfig {
    type P2P: P2PConfig;
}
