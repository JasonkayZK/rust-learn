use libp2p::floodsub::{Floodsub, FloodsubEvent};
use libp2p::mdns;
use libp2p::swarm::NetworkBehaviour;

#[derive(NetworkBehaviour)]
#[behaviour(to_swarm = "RecipeBehaviourEvent")]
pub struct RecipeBehaviour {
    pub(crate) flood_sub: Floodsub,
    pub(crate) mdns: mdns::tokio::Behaviour,
}

#[derive(Debug)]
pub enum RecipeBehaviourEvent {
    Floodsub(FloodsubEvent),
    Mdns(mdns::Event),
}

impl From<FloodsubEvent> for RecipeBehaviourEvent {
    fn from(event: FloodsubEvent) -> RecipeBehaviourEvent {
        RecipeBehaviourEvent::Floodsub(event)
    }
}

impl From<mdns::Event> for RecipeBehaviourEvent {
    fn from(event: mdns::Event) -> RecipeBehaviourEvent {
        RecipeBehaviourEvent::Mdns(event)
    }
}
