use libp2p::swarm::NetworkBehaviour;
use libp2p::{gossipsub, mdns};

#[derive(NetworkBehaviour)]
#[behaviour(to_swarm = "RecipeBehaviourEvent")]
pub struct RecipeBehaviour {
    pub(crate) gossip: gossipsub::Behaviour,
    pub(crate) mdns: mdns::tokio::Behaviour,
}

#[derive(Debug)]
pub enum RecipeBehaviourEvent {
    Gossip(gossipsub::Event),
    Mdns(mdns::Event),
}

impl From<gossipsub::Event> for RecipeBehaviourEvent {
    fn from(event: gossipsub::Event) -> RecipeBehaviourEvent {
        RecipeBehaviourEvent::Gossip(event)
    }
}

impl From<mdns::Event> for RecipeBehaviourEvent {
    fn from(event: mdns::Event) -> RecipeBehaviourEvent {
        RecipeBehaviourEvent::Mdns(event)
    }
}
