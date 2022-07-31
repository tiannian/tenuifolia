use primitive_types::{H160, H256};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct BlockHash(pub H256);

pub struct MerkleHash(pub H256);

pub struct Timestamp(pub i64);

pub struct NodeId(pub H160);
