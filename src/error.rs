use libp2p::{identity, multiaddr, multihash};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    IoError(#[from] std::io::Error),

    #[error(transparent)]
    DecodeError(#[from] identity::error::DecodingError),

    #[error(transparent)]
    MultiAddrError(#[from] multiaddr::Error),

    #[error(transparent)]
    UdsError(#[from] libp2p::TransportError<std::io::Error>),

    #[error("Must have least one peer key pair.")]
    AtLeastOnePeerKeypair,

    #[error(transparent)]
    MultiHashError(#[from] multihash::Error),

    #[error("gossip error: {0}")]
    GossipError(&'static str),

    #[error("grandpa consensus is not descendent")]
    GrandpaNotDescendent,

    #[error("async std try send error")]
    TrySenderError,

    #[error("It's unkonwn error")]
    UnknownError,
}

impl From<Error> for finality_grandpa::Error {
    fn from(e: Error) -> finality_grandpa::Error {
        match e {
            Error::GrandpaNotDescendent => finality_grandpa::Error::NotDescendent,
            _ => panic!("Error convert failed, unexpected error: {:?}", e),
        }
    }
}

impl From<finality_grandpa::Error> for Error {
    fn from(e: finality_grandpa::Error) -> Self {
        match e {
            finality_grandpa::Error::NotDescendent => Self::GrandpaNotDescendent,
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;
