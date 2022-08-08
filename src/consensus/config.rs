pub struct Config {
    pub prevote_delay_millis: u64,
    pub precommit_delay_millis: u64,
    pub commit_delay_millis: u64,
    pub peer_id: Option<Vec<u8>>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            prevote_delay_millis: 1000,
            precommit_delay_millis: 1000,
            commit_delay_millis: 1000,
            peer_id: None,
        }
    }
}
