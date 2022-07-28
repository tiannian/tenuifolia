pub mod p2p;

mod error;
pub use error::*;

/// Core type.
pub mod core;

/// Message.
pub mod message;

mod peer_key;
pub use peer_key::*;

mod prelude;
pub use prelude::*;

pub struct ValidatorConfig;

impl NodeTypeConfig for ValidatorConfig {
    type P2P = p2p::behaviour::ValiatorNetworkBehaviour;
}

