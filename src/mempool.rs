use std::sync::Arc;

use crate::{Entity, Result};

pub struct Mempool<ChainApp, MempoolApp> {
    chain: Arc<ChainApp>,
    mempool: MempoolApp,
}

impl<ChainApp, MempoolApp> Mempool<ChainApp, MempoolApp> {
    pub fn new(chain: Arc<ChainApp>, mempool: MempoolApp) -> Self {
        Self { chain, mempool }
    }

    pub fn add_entity(&mut self, entity: impl Entity<ChainApp, MempoolApp>) -> Result<()> {
        Ok(())
    }
}
