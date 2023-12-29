use libp2p::{gossipsub, mdns, Swarm};
use libp2p::futures::StreamExt;
use libp2p::swarm::SwarmEvent;
use log::{debug, info};
use tokio::sync::mpsc;

use crate::behaviour::{RecipeBehaviour, RecipeBehaviourEvent};
use crate::models::ListResponse;

mod mdns_event;
mod swarm_event;
mod gossip_event;

pub async fn handle_swarm_event(
    response_sender: mpsc::UnboundedSender<ListResponse>,
    swarm: &mut Swarm<RecipeBehaviour>,
) {
    let event = swarm.select_next_some().await;
    info!("Income swarm Event: {:?}", event);

    match event {
        SwarmEvent::Behaviour(recipe_behaviours) => match recipe_behaviours {
            RecipeBehaviourEvent::Gossip(gossip_event) => match gossip_event {
                gossipsub::Event::Message { propagation_source, message, .. } => gossip_event::handle_message(propagation_source, message, response_sender).await,
                gossipsub::Event::Subscribed { .. } => {}
                gossipsub::Event::Unsubscribed { .. } => {}
                gossipsub::Event::GossipsubNotSupported { .. } => {}
            }

            RecipeBehaviourEvent::Mdns(mdns_event) => match mdns_event {
                mdns::Event::Discovered(discovered_list) => mdns_event::handle_discovered(swarm, discovered_list).await,
                mdns::Event::Expired(expired_list) => mdns_event::handle_expired(swarm, expired_list).await,
            },
        },
        SwarmEvent::ConnectionEstablished {
            peer_id,
            connection_id,
            endpoint,
            num_established,
            ..
        } => {
            debug!("[Connection established] peer_id: {}, connection_id: {}, endpoint: {:?}, num_established: {:?}", peer_id, connection_id, endpoint, num_established);
            swarm_event::handle_connection_established(swarm, peer_id).await;
        }
        SwarmEvent::ConnectionClosed {
            peer_id,
            connection_id,
            endpoint,
            num_established,
            ..
        } => {
            debug!("[Connection closed] peer_id: {}, connection_id: {}, endpoint: {:?}, num_established: {:?}", peer_id, connection_id, endpoint, num_established);
        }
        SwarmEvent::IncomingConnection { .. } => {}
        SwarmEvent::IncomingConnectionError { .. } => {}
        SwarmEvent::OutgoingConnectionError { .. } => {}
        SwarmEvent::NewListenAddr { .. } => {}
        SwarmEvent::ExpiredListenAddr { .. } => {}
        SwarmEvent::ListenerClosed { .. } => {}
        SwarmEvent::ListenerError { .. } => {}
        SwarmEvent::Dialing { .. } => {}
    };
}
