use finality_grandpa::{voter::Environment, Chain};

use crate::{core::BlockHash, Store};

pub struct GrandpaEnvironment<S: Store> {
    store: S,
}

impl<S: Store> Chain<BlockHash, u64> for GrandpaEnvironment<S> {
    fn ancestry(&self, base: BlockHash, end: BlockHash) -> Result<Vec<BlockHash>, finality_grandpa::Error> {
        let hashs = self.store.get_blockhash_sequence(base, end)?;
        Ok(hashs)
    }
}

impl<S: Store> Environment<BlockHash, u64> for GrandpaEnvironment<S> {

}

