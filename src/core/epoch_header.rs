use digest::Digest;

use super::{EpochHash, MerkleHash, NodeId, Timestamp};

pub struct EpochHeader {
    pub height: u64,
    pub timestamp: Timestamp,
    pub parent_hash: EpochHash,
    pub app_hash: MerkleHash,
    pub entity_merkle: MerkleHash,
    pub proposer: NodeId,
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

pub struct Signature {
    pub node_id_index: u32,
    pub public_key_index: u32,
    pub signature: Vec<u8>,
}

pub struct Signatures {
    pub signatures: Vec<Signature>,
}

pub enum PublicKey {
    Ed25519(Vec<u8>),
    Secp256k1(Vec<u8>),
}

pub struct ValidatorInfo {
    pub node_id: NodeId,
    pub weight: u64,
    pub public_keys: Vec<PublicKey>,
}

pub struct ValidatorSet {
    pub set: Vec<ValidatorInfo>,
}
