#[derive(Debug, Clone)]
pub enum DelayType {
    Static,
    Rate([u64; 2]),
}

#[derive(Debug, Clone)]
pub struct Config {
    pub prevote_delay_millis: u64,
    pub precommit_delay_millis: u64,
    pub commit_delay_millis: u64,
    pub peer_id: Option<Vec<u8>>,
    pub delay_type: DelayType,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            prevote_delay_millis: 1000,
            precommit_delay_millis: 1000,
            commit_delay_millis: 1000,
            peer_id: None,
            delay_type: DelayType::Static,
        }
    }
}
