use std::time::Duration;

use finality_grandpa::{
    round::State, voter::Environment, Chain, Commit, Equivocation, HistoricalVotes, Precommit,
    Prevote, PrimaryPropose,
};

use crate::{core::EpochHash, EpochPacker, Error, Mempool, PeerSignature, Store};

use super::{best_chain::BestChain, timer::Timer};

pub struct GrandpaEnvironment<S, Packer, ChainApp, MempoolApp> {
    store: S,
    packer: Packer,
    mempool: Mempool<ChainApp, MempoolApp>,
}

impl<S: Store, Packer, ChainApp, MempoolApp> Chain<EpochHash, u64>
    for GrandpaEnvironment<S, Packer, ChainApp, MempoolApp>
{
    fn ancestry(
        &self,
        base: EpochHash,
        end: EpochHash,
    ) -> Result<Vec<EpochHash>, finality_grandpa::Error> {
        let hashs = self.store.get_epoch_hash_sequence(base, end)?;
        Ok(hashs)
    }
}

impl<S: Store, Packer: EpochPacker<ChainApp, MempoolApp>, ChainApp, MempoolApp>
    Environment<EpochHash, u64> for GrandpaEnvironment<S, Packer, ChainApp, MempoolApp>
{
    type Error = Error;

    type Timer = Timer;

    type Id = Vec<u8>;

    type Signature = PeerSignature;

    type BestChain = BestChain;

    // type In = Box<
    //     dyn Stream<Item = Result<SignedMessage<BlockHash, u64, Self::Signature, Self::Id>, Error>>
    //         + Unpin
    //         + Send,
    //     >;

    // type Out = Pin<Box<dyn Sink<Message<BlockHash, u64>, Error = Error> + Send>>;

    fn best_chain_containing(&self, base: EpochHash) -> Self::BestChain {
        let block = async move {
            let epoch_header = self.packer.pack(&self.mempool).await?;

            Ok(Some((EpochHash::default(), 0)))
        };

        Self::BestChain::new(block)
    }

    fn round_commit_timer(&self) -> Self::Timer {
        use rand::Rng;

        const COMMIT_DELAY_MILLIS: u64 = 1000;

        let thread_rng = rand::thread_rng();

        let delay = Duration::from_millis(thread_rng.gen_range(0..COMMIT_DELAY_MILLIS));

        Timer::sleep(delay)
    }

    fn proposed(&self, round: u64, propose: PrimaryPropose<EpochHash, u64>) -> Result<(), Error> {
        Ok(())
    }

    fn prevoted(&self, round: u64, prevote: Prevote<EpochHash, u64>) -> Result<(), Error> {
        Ok(())
    }

    fn precommitted(&self, round: u64, precommit: Precommit<EpochHash, u64>) -> Result<(), Error> {
        Ok(())
    }

    fn completed(
        &self,
        round: u64,
        state: State<EpochHash, u64>,
        base: (EpochHash, u64),
        votes: &HistoricalVotes<EpochHash, u64, Self::Signature, Self::Id>,
    ) -> Result<(), Error> {
        Ok(())
    }

    fn concluded(
        &self,
        round: u64,
        state: State<EpochHash, u64>,
        base: (EpochHash, u64),
        votes: &HistoricalVotes<EpochHash, u64, Self::Signature, Self::Id>,
    ) -> Result<(), Error> {
        Ok(())
    }

    fn finalize_block(
        &self,
        hash: EpochHash,
        number: u64,
        round: u64,
        commit: Commit<EpochHash, u64, Self::Signature, Self::Id>,
    ) -> Result<(), Error> {
        Ok(())
    }

    fn prevote_equivocation(
        &self,
        round: u64,
        equivocation: Equivocation<Self::Id, Prevote<EpochHash, u64>, Self::Signature>,
    ) {
    }

    fn precommit_equivocation(
        &self,
        round: u64,
        equivocation: Equivocation<Self::Id, Precommit<EpochHash, u64>, Self::Signature>,
    ) {
    }
}
