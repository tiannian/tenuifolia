use libp2p::{
    gossipsub::{Gossipsub, GossipsubEvent},
    kad::{store::MemoryStore, Kademlia, KademliaEvent},
    request_response::{RequestResponse, RequestResponseEvent},
    NetworkBehaviour,
};

use crate::message::Message;

use super::req_resp::Codec;

#[derive(NetworkBehaviour)]
#[behaviour(out_event = "ComposedEvent")]
pub struct ValidatorNetworkBehaviour {
    pub kad: Kademlia<MemoryStore>,
    pub re: RequestResponse<Codec>,
    pub gossip: Gossipsub,
}

#[derive(Debug)]
pub enum ComposedEvent {
    RequestResponse(RequestResponseEvent<Message, Message>),
    Kademlia(KademliaEvent),
    Gossipsub(GossipsubEvent),
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

impl From<GossipsubEvent> for ComposedEvent {
    fn from(e: GossipsubEvent) -> Self {
        ComposedEvent::Gossipsub(e)
    }
}
