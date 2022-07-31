use std::{future::Future, pin::Pin};

use finality_grandpa::{voter::Environment, Chain, SignedMessage, Message};
use libp2p::futures::{Stream, Sink};

use crate::{core::BlockHash, Error, PeerSignature, Store};

pub struct GrandpaEnvironment<S: Store> {
    store: S,
}

impl<S: Store> Chain<BlockHash, u64> for GrandpaEnvironment<S> {
    fn ancestry(
        &self,
        base: BlockHash,
        end: BlockHash,
    ) -> Result<Vec<BlockHash>, finality_grandpa::Error> {
        let hashs = self.store.get_blockhash_sequence(base, end)?;
        Ok(hashs)
    }
}

impl<S: Store> Environment<BlockHash, u64> for GrandpaEnvironment<S> {
    type Error = Error;

    type Timer = Box<dyn Future<Output = Result<(), Error>> + Unpin + Send>;

    type Id = Vec<u8>;

    type Signature = PeerSignature;

    type BestChain =
        Box<dyn Future<Output = Result<Option<(BlockHash, u64)>, Error>> + Unpin + Send>;

    type In = Box<
        dyn Stream<Item = Result<SignedMessage<BlockHash, u64, Self::Signature, Self::Id>, Error>>
            + Unpin
            + Send,
    >;

    type Out = Pin<Box<dyn Sink<Message<BlockHash, u64>, Error = Error> + Send>>;
}
