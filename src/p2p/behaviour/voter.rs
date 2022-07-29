use std::{iter, time::Duration};

use libp2p::{
    kad::{store::MemoryStore, Kademlia, KademliaConfig, KademliaEvent},
    request_response::{
        ProtocolSupport, RequestResponse, RequestResponseConfig, RequestResponseEvent,
    },
    NetworkBehaviour, PeerId,
};

use crate::{
    message::Message,
    p2p::{config::Config, req_resp},
    P2PConfig, PeerKeys, Result,
};

#[derive(NetworkBehaviour)]
#[behaviour(out_event = "VoterComposedEvent")]
pub struct VoterNetworkBehaviour {
    pub kad: Kademlia<MemoryStore>,
    pub re: RequestResponse<req_resp::Codec>,
}

impl P2PConfig for VoterNetworkBehaviour {
    fn new(config: &Config, keys: &PeerKeys) -> Result<Self> {
        let mut kad_config = KademliaConfig::default();

        kad_config.disjoint_query_paths(config.kademlia.enable_disjoint_query_paths);

        let local_id = PeerId::from_bytes(&keys.peer_id)?;

        let kad = Kademlia::with_config(local_id, MemoryStore::new(local_id), kad_config);

        let mut re_config = RequestResponseConfig::default();

        let timeout = Duration::from_secs(config.re2.request_timeout);

        re_config.set_request_timeout(timeout);

        let re = RequestResponse::new(
            req_resp::Codec,
            iter::once((req_resp::Protocol, ProtocolSupport::Full)),
            re_config,
        );

        Ok(Self { kad, re })
    }
}

#[derive(Debug)]
pub enum VoterComposedEvent {
    RequestResponse(RequestResponseEvent<Message, Message>),
    Kademlia(KademliaEvent),
}
impl From<RequestResponseEvent<Message, Message>> for VoterComposedEvent {
    fn from(event: RequestResponseEvent<Message, Message>) -> Self {
        Self::RequestResponse(event)
    }
}

impl From<KademliaEvent> for VoterComposedEvent {
    fn from(event: KademliaEvent) -> Self {
        Self::Kademlia(event)
    }
}
