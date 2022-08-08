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

pub mod consensus;

mod mempool;
pub use mempool::*;

mod store;
pub use store::*;

pub struct VoterConfig;

impl NodeTypeConfig for VoterConfig {
    type P2P = p2p::behaviour::VoterNetworkBehaviour;
}

pub struct FullConfig;

impl NodeTypeConfig for FullConfig {
    type P2P = p2p::behaviour::FullNetworkBehaviour;
}
