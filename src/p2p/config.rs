use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Transport {
    Tcp,
    Websocket,
    Uds,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub transport: Vec<Transport>,
    pub bootstrap_nodes: Vec<String>,
    pub peer_key: PeerKey,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PeerKey {
    pub peer_id: String,
    pub public_key: String,
    pub secret_key: String,
}

