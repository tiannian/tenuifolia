use libp2p::{Swarm, kad::{Kademlia, store::MemoryStore}, request_response::RequestResponse};
use serde::{Serialize, Deserialize};

use crate::Result;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Transport {
    Tcp,
    Websocket,
    Uds,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub transport: Vec<Transport>,
    pub bootstrap_nodes: Vec<String>,
    pub peer_key: PeerKey,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PeerKey {
    pub peer_id: String,
    pub public_key: String,
    pub secret_key: String,
}

pub struct ValidatorNetworkBehaviour {
    pub kad: Kademlia<MemoryStore>,
    // pub re: RequestResponse<>,
}

pub struct P2P {
    // pub swarm: Swarm<>,
}

impl P2P {
    pub fn new(config: Config) -> Result<Self> {


        Ok(Self {})
    }
}

