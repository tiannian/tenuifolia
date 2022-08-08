use std::{time::Duration, sync::Arc};

use finality_grandpa::{
    round::State,
    voter::{Environment, RoundData},
    Chain, Commit, Equivocation, HistoricalVotes, Message, Precommit, Prevote, PrimaryPropose,
    SignedMessage,
};
use futures::channel::mpsc::UnboundedReceiver;

use crate::{
    core::EpochHash, p2p::channel::Sender, EpochPacker, Error, Mempool, PeerSignature, Store,
};

use super::{best_chain::BestChain, timer::Timer, Config, channel::ConsensusSide};

pub struct GrandpaEnvironment<S, Packer, ChainApp, MempoolApp> {
    store: S,
    packer: Packer,
    mempool: Mempool<ChainApp, MempoolApp>,
    config: Config,
    side: ConsensusSide,
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

impl<
        S: Store,
        Packer: EpochPacker<ChainApp, MempoolApp>,
        ChainApp: Send + Sync,
        MempoolApp: Send + Sync,
    > Environment<EpochHash, u64> for GrandpaEnvironment<S, Packer, ChainApp, MempoolApp>
{
    type Error = Error;

    type Timer = Timer;

    type Id = Vec<u8>;

    type Signature = PeerSignature;

    type BestChain = BestChain;

    type In =
        Arc<UnboundedReceiver<Result<SignedMessage<EpochHash, u64, PeerSignature, Vec<u8>>, Error>>>;

    type Out = Sender<Message<EpochHash, u64>>;

    fn best_chain_containing(&self, base: EpochHash) -> Self::BestChain {
        let block = async {
            let mempool = &self.mempool;

            let epoch_header = self.packer.pack(mempool).await?;

            let hash = epoch_header.hash::<Packer::Digest>();

            Ok(Some((hash, epoch_header.height)))
        };

        Self::BestChain::new(block)
    }

    fn round_data(&self, round: u64) -> RoundData<Self::Id, Self::Timer, Self::In, Self::Out> {
        use rand::Rng;

        let thread_rng = rand::thread_rng();
        let delay =
            Duration::from_millis(thread_rng.gen_range(0..self.config.prevote_delay_millis));
        let prevote_timer = Timer::sleep(delay);

        let thread_rng = rand::thread_rng();
        let delay =
            Duration::from_millis(thread_rng.gen_range(0..self.config.prevote_delay_millis));
        let precommit_timer = Timer::sleep(delay);

        RoundData {
            voter_id: self.config.peer_id.clone(),
            prevote_timer,
            precommit_timer,
            incoming: self.side.grandpa_in.clone(),
            outgoing: self.side.grandpa_out.clone(),
        }
    }

    fn round_commit_timer(&self) -> Self::Timer {
        use rand::Rng;

        let thread_rng = rand::thread_rng();

        let delay = Duration::from_millis(thread_rng.gen_range(0..self.config.commit_delay_millis));

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
