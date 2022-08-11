use digest::{Digest, Output};
use quick_merkle::Merkle;

use super::{MerkleHash, NodeId, PublicKey};

pub struct VoterInfo {
    pub node_id: NodeId,
    pub weight: u64,
    pub public_keys: Vec<PublicKey>,
}

impl VoterInfo {
    pub fn hash<D: Digest>(&self) -> Output<D> {
        let mut hasher = D::new();

        hasher.update(self.node_id.clone());
        hasher.update(self.weight.to_be_bytes());

        for pk in &self.public_keys {
            hasher.update(pk.hash::<D>());
        }

        hasher.finalize()
    }
}

pub struct VoterSet {
    pub set: Vec<VoterInfo>,
}

impl VoterSet {
    pub fn hash<D: Digest>(&self) -> MerkleHash {
        let leafs = self.set.iter().map(|e| e.hash::<D>()).collect();

        let merkle = Merkle::<D>::new(leafs);

        if let Some(m) = merkle {
            MerkleHash::from_bytes(m.root())
        } else {
            Default::default()
        }
    }
}
