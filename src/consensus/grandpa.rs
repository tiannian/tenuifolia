use std::time::Duration;

use async_std::channel::Receiver;
use finality_grandpa::{
    round::State,
    voter::{Environment, RoundData},
    Chain, Commit, Equivocation, HistoricalVotes, Message, Precommit, Prevote, PrimaryPropose,
    SignedMessage,
};

use crate::{core::EpochHash, EpochPacker, Error, PeerSignature, Store, consensus::DelayType};

use super::{
    best_chain::BestChain,
    channel::{ConsensusSide, SinkSender},
    timer::Timer,
    Config,
};

pub struct GrandpaEnvironment<S, Packer> {
    store: S,
    packer: Packer,
    config: Config,
    side: ConsensusSide,
}

impl<S: Store, Packer> Chain<EpochHash, u64> for GrandpaEnvironment<S, Packer> {
    fn ancestry(
        &self,
        base: EpochHash,
        end: EpochHash,
    ) -> Result<Vec<EpochHash>, finality_grandpa::Error> {
        let hashs = self.store.get_epoch_hash_sequence(base, end)?;
        Ok(hashs)
    }
}

impl<S: Store, Packer: EpochPacker> Environment<EpochHash, u64> for GrandpaEnvironment<S, Packer> {
    type Error = Error;

    type Timer = Timer;

    type Id = Vec<u8>;

    type Signature = PeerSignature;

    type BestChain = BestChain;

    type In = Receiver<Result<SignedMessage<EpochHash, u64, PeerSignature, Vec<u8>>, Error>>;

    type Out = SinkSender<Message<EpochHash, u64>>;

    fn best_chain_containing(&self, _base: EpochHash) -> Self::BestChain {
        let packer = self.packer.clone();

        // Checking base

        let block = async move {
            let epoch_header = packer.pack().await?;

            let hash = epoch_header.hash::<Packer::Digest>();

            Ok(Some((hash, epoch_header.height)))
        };

        Self::BestChain::new(block)
    }

    fn round_data(&self, round: u64) -> RoundData<Self::Id, Self::Timer, Self::In, Self::Out> {
        use rand::Rng;

        fn build_timer(round: u64, delay: u64, ty: DelayType) -> Timer {
            let (begin, end) = match ty {
                DelayType::Static => {
                    (0, delay)
                }
                DelayType::Rate(r) => {
                    if round == 0 {
                        (0, delay)
                    } else {
                        let begin = delay * round;

                        let offset = delay * r[0] / r[1];

                        (begin, begin + offset)
                    }
                }
            };

            let mut thread_rng = rand::thread_rng();
            let delay =
                Duration::from_millis(thread_rng.gen_range(begin .. end));
            Timer::sleep(delay)
        }

        let prevote_timer = build_timer(round, self.config.prevote_delay_millis, self.config.delay_type.clone());
        let precommit_timer = build_timer(round, self.config.precommit_delay_millis, self.config.delay_type.clone());

        RoundData {
            voter_id: self.config.peer_id.clone(),
            prevote_timer,
            precommit_timer,
            incoming: self.side.grandpa_in.clone(),
            outgoing: self.side.grandpa_out.clone().into(),
        }
    }

    fn round_commit_timer(&self) -> Self::Timer {
        use rand::Rng;

        let mut thread_rng = rand::thread_rng();

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
