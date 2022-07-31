use crate::{core::EpochHash, Result};

pub trait Store {
    fn get_epoch_hash_by_height(&self, height: u64) -> Result<EpochHash>;

    fn get_epoch_hash_by_height_batch(&self, begin: u64, end: u64) -> Result<Vec<EpochHash>>;

    fn get_epoch_hash_sequence(&self, begin: EpochHash, end: EpochHash) -> Result<Vec<EpochHash>>;

    // fn get_value_by_block()
}
