use std::{time::Duration, iter};

use libp2p::{
    kad::{store::MemoryStore, Kademlia, KademliaEvent, KademliaConfig},
    request_response::{RequestResponse, RequestResponseEvent, RequestResponseConfig, ProtocolSupport},
    NetworkBehaviour, PeerId,
};

use crate::{message::Message, P2PConfig, PeerKeys, Result};

use super::req_resp::{Codec, self};


#[derive(NetworkBehaviour)]
#[behaviour(out_event = "ComposedEvent")]
pub struct ValidatorNetworkBehaviour {
    pub kad: Kademlia<MemoryStore>,
    pub re: RequestResponse<Codec>,
}

impl P2PConfig for ValidatorNetworkBehaviour {
    fn new(config: &super::config::Config, keys: &PeerKeys) -> Result<Self> {
        let mut kad_config = KademliaConfig::default();

        kad_config.disjoint_query_paths(config.kademlia.enable_disjoint_query_paths);

        let local_id = PeerId::from_bytes(&keys.peer_id)?;

        let kad = Kademlia::with_config(local_id, MemoryStore::new(local_id), kad_config);

        let mut re_config = RequestResponseConfig::default();

        let timeout = Duration::from_secs(config.re2.request_timeout);

        re_config.set_request_timeout(timeout);

        let re = RequestResponse::new(req_resp::Codec, iter::once((req_resp::Protocol, ProtocolSupport::Full)), re_config);

        Ok(Self {
            kad, re
        })
    }
}

#[derive(Debug)]
pub enum ComposedEvent {
    RequestResponse(RequestResponseEvent<Message, Message>),
    Kademlia(KademliaEvent),
}
impl From<RequestResponseEvent<Message, Message>> for ComposedEvent {
    fn from(event: RequestResponseEvent<Message, Message>) -> Self {
        ComposedEvent::RequestResponse(event)
    }
}

impl From<KademliaEvent> for ComposedEvent {
    fn from(event: KademliaEvent) -> Self {
        ComposedEvent::Kademlia(event)
    }
}

