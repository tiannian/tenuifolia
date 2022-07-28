pub mod config;

mod req_resp;

mod behaviour;

use crate::Result;

pub struct P2P {
    // pub swarm: Swarm<>,
}

impl P2P {
    pub fn new(config: config::Config) -> Result<Self> {
        Ok(Self {})
    }
}
