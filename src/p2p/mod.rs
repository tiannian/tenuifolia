use libp2p::{Swarm, kad::{Kademlia, store::MemoryStore}, request_response::RequestResponse};
use crate::Result;

pub mod config;

pub struct ValidatorNetworkBehaviour {
    pub kad: Kademlia<MemoryStore>,
    // pub re: RequestResponse<>,
}

pub struct P2P {
    // pub swarm: Swarm<>,
}

impl P2P {
    pub fn new(config: config::Config) -> Result<Self> {


        Ok(Self {})
    }
}

