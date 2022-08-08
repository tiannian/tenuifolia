use crate::{
    core::{EpochHash, EpochHeader},
    Result,
};

pub trait Store: Send + Sync {
    fn get_epoch_hash_by_height(&self, height: u64) -> Result<EpochHash>;

    fn get_epoch_hash_by_height_batch(&self, begin: u64, end: u64) -> Result<Vec<EpochHash>>;

    fn get_epoch_hash_sequence(&self, begin: EpochHash, end: EpochHash) -> Result<Vec<EpochHash>>;

    fn set_epoch_header(&self, hash: EpochHash, header: EpochHeader) -> Result<()>;

    fn set_epoch_hash_height(&self, hash: EpochHeader, height: u64) -> Result<()>;

    // fn get_value_by_block()
}
