use std::io;

use libp2p::{identity, multiaddr, multihash};

#[derive(Debug)]
pub enum Error {
    IoError(io::Error),
    DecodeError(identity::error::DecodingError),
    MultiAddrError(multiaddr::Error),
    UdsError(libp2p::TransportError<io::Error>),
    AtLeastOnePeerKeypair,
    MultiHashError(multihash::Error),
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::IoError(e)
    }
}

impl From<identity::error::DecodingError> for Error {
    fn from(e: identity::error::DecodingError) -> Self {
        Error::DecodeError(e)
    }
}

impl From<multiaddr::Error> for Error {
    fn from(e: multiaddr::Error) -> Self {
        Error::MultiAddrError(e)
    }
}

impl From<libp2p::TransportError<io::Error>> for Error {
    fn from(e: libp2p::TransportError<io::Error>) -> Self {
        Error::UdsError(e)
    }
}

impl From<multihash::Error> for Error {
    fn from(e: multihash::Error) -> Self {
        Error::MultiHashError(e)
    }
}

pub type Result<T> = std::result::Result<T, Error>;
