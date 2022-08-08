use primitive_types::{H160, H256};

macro_rules! define_for_primitive_type {
    ($main:ty, $inner:ty) => {
        impl $main {
            pub fn from_bytes(bytes: &impl AsRef<[u8]>) -> Self {
                let inner = <$inner>::from_slice(bytes.as_ref());

                Self(inner)
            }

            pub fn as_bytes(&self) -> &[u8] {
                self.0.as_bytes()
            }
        }
    };
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct EpochHash(pub H256);
define_for_primitive_type!(EpochHash, H256);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct MerkleHash(pub H256);
define_for_primitive_type!(MerkleHash, H256);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct NodeId(pub H160);
define_for_primitive_type!(NodeId, H160);

macro_rules! define_for_core_type {
    ($main:ty, $inner:ty, $len:expr) => {
        impl $main {
            pub fn from_bytes(bytes: [u8; $len]) -> Self {
                let inner = <$inner>::from_be_bytes(bytes);

                Self(inner)
            }

            pub fn as_bytes(&self) -> &[u8] {
                &self.0.to_be_bytes()
            }
        }
    };
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Timestamp(pub i64);
define_for_core_type!(Timestamp, i64, 8);
