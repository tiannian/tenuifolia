use digest::Digest;
use quick_merkle::Merkle;

use crate::{Entity, EpochPacker};

use super::{EntityBody, EpochHash, MerkleHash, NodeId, Timestamp, VoterSet};

pub struct EpochHeader {
    pub height: u64,
    pub timestamp: Timestamp,
    pub parent_hash: EpochHash,
    pub app_hash: MerkleHash,
    pub entity_merkle: MerkleHash,
    pub proposer: NodeId,
    pub voter_merkle: MerkleHash,
}

impl EpochHeader {
    pub fn hash<D: Digest>(&self) -> EpochHash {
        let mut hasher = D::new();

        hasher.update(self.height.to_be_bytes());
        hasher.update(self.timestamp.as_bytes());
        hasher.update(self.parent_hash.as_bytes());
        hasher.update(self.app_hash.as_bytes());
        hasher.update(self.entity_merkle.as_bytes());
        hasher.update(self.proposer.as_bytes());

        EpochHash::from_bytes(&hasher.finalize())
    }
}

pub struct PackerEpoch<P: EpochPacker> {
    pub height: u64,
    pub timestamp: Timestamp,
    pub parent_hash: EpochHash,
    pub app_hash: MerkleHash,
    pub entities: Vec<P::Entity>,
    pub voter_set: VoterSet,
}

impl<P: EpochPacker> PackerEpoch<P> {
    pub fn build(self, node_id: NodeId) -> (EpochHeader, Vec<EntityBody>, VoterSet) {
        let ebs: Vec<EntityBody> = self.entities.iter().map(|e| e.to_body()).collect();

        let ehs = ebs.iter().map(|e| e.hash::<P::Digest>()).collect();

        let merkle = Merkle::<P::Digest>::new(ehs);

        let emr = if let Some(m) = merkle {
            *m.root()
        } else {
            Default::default()
        };

        let vs = self.voter_set;

        let header = EpochHeader {
            height: self.height,
            timestamp: self.timestamp,
            parent_hash: self.parent_hash,
            app_hash: self.app_hash,
            proposer: node_id,
            voter_merkle: vs.hash::<P::Digest>(),
            entity_merkle: MerkleHash::from_bytes(&emr),
        };

        (header, ebs, vs)
    }
}
