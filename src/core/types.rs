use primitive_types::{H160, H256};

pub struct BlockHash(pub H256);

pub struct BlockHeight(pub u64);

pub struct MerkleHash(pub H256);

pub struct Timestamp(pub i64);

pub struct NodeId(pub H160);
