pub mod config;

mod req_resp;

pub(crate) mod behaviour;

use libp2p::{PeerId, Swarm};

use crate::{Error, NodeTypeConfig, P2PConfig, PeerKeys, Result};

pub struct P2P<Config: NodeTypeConfig> {
    pub(crate) swarm: Swarm<Config::P2P>,
}

impl<Config: NodeTypeConfig> P2P<Config> {
    pub fn new(config: config::Config, keys: &PeerKeys) -> Result<Self> {
        let keypair = keys.keypairs.get(0).ok_or(Error::AtLeastOnePeerKeypair)?;

        let transport = config.build_transport(keypair)?;

        let behaviour = <Config::P2P as P2PConfig>::new(&config, keys)?;

        let peer_id = PeerId::from_bytes(&keys.peer_id)?;

        let swarm = Swarm::new(transport, behaviour, peer_id);

        Ok(Self { swarm })
    }
}
