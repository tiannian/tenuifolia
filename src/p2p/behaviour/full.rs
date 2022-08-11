use std::{iter, time::Duration};

use libp2p::{
    gossipsub::{Gossipsub, GossipsubEvent},
    gossipsub::{GossipsubConfig, MessageAuthenticity},
    kad::{store::MemoryStore, Kademlia, KademliaConfig, KademliaEvent},
    request_response::{
        ProtocolSupport, RequestResponse, RequestResponseConfig, RequestResponseEvent,
    },
    NetworkBehaviour, PeerId,
};

use crate::{
    message::Message,
    p2p::{self, req_resp},
    Error, P2PConfig, PeerKeys, Result,
};

#[derive(NetworkBehaviour)]
#[behaviour(out_event = "FullComposedEvent")]
pub struct FullNetworkBehaviour {
    pub kad: Kademlia<MemoryStore>,
    pub re: RequestResponse<req_resp::Codec>,
    pub gossip: Gossipsub,
}

impl P2PConfig for FullNetworkBehaviour {
    fn new(config: &p2p::config::Config, keys: &PeerKeys) -> Result<Self> {
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

        let message_authenticity = MessageAuthenticity::Anonymous;

        let go_config = GossipsubConfig::default();

        let gossip = Gossipsub::new(message_authenticity, go_config).map_err(Error::GossipError)?;

        Ok(Self { kad, re, gossip })
    }
}

#[derive(Debug)]
pub enum FullComposedEvent {
    RequestResponse(RequestResponseEvent<Message, Message>),
    Kademlia(KademliaEvent),
    Gossip(GossipsubEvent),
}
impl From<RequestResponseEvent<Message, Message>> for FullComposedEvent {
    fn from(event: RequestResponseEvent<Message, Message>) -> Self {
        FullComposedEvent::RequestResponse(event)
    }
}

impl From<KademliaEvent> for FullComposedEvent {
    fn from(event: KademliaEvent) -> Self {
        FullComposedEvent::Kademlia(event)
    }
}

impl From<GossipsubEvent> for FullComposedEvent {
    fn from(e: GossipsubEvent) -> Self {
        FullComposedEvent::Gossip(e)
    }
}
