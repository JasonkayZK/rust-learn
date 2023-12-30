use libp2p::{gossipsub, mdns, Swarm};
use libp2p::futures::StreamExt;
use libp2p::swarm::SwarmEvent;
use log::info;
use tokio::sync::mpsc;

use crate::behaviour::{RecipeBehaviour, RecipeBehaviourEvent};
use crate::consts::INIT_SYNC_STR;
use crate::models::ListResponse;
use crate::sync::progress_manager::ProgressManager;

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
                gossipsub::Event::Message { propagation_source, message, .. } => gossip_event::handle_message(swarm, propagation_source, message, response_sender).await,
                gossipsub::Event::Subscribed { peer_id, topic } => {
                    let topic_id = topic.to_string();

                    if topic_id.eq(INIT_SYNC_STR) {
                        // For those new subscribers who is the new peers joined in the network, we should sync data
                        ProgressManager::init_sync_data(swarm, peer_id).await;
                    }
                }
                gossipsub::Event::Unsubscribed { peer_id, topic } => {
                    let topic_id = topic.to_string();

                    if topic_id.eq(INIT_SYNC_STR) {
                        // For those subscribers who exit from the network, we should stop sync data
                        ProgressManager::stop_sync_data(swarm, peer_id).await;
                    }
                }
                gossipsub::Event::GossipsubNotSupported { .. } => {}
            }

            RecipeBehaviourEvent::Mdns(mdns_event) => match mdns_event {
                mdns::Event::Discovered(discovered_list) => mdns_event::handle_discovered(swarm, discovered_list).await,
                mdns::Event::Expired(expired_list) => mdns_event::handle_expired(swarm, expired_list).await,
            },
        },
        // Because bi-directional connection will be established, so we will get two ConnectionEstablished events when one peer joined!
        SwarmEvent::ConnectionEstablished {
            peer_id,
            connection_id,
            endpoint,
            num_established,
            ..
        } => {
            info!("[Connection established] peer_id: {}, connection_id: {}, endpoint: {:?}, num_established: {:?}", peer_id, connection_id, endpoint, num_established);

            // Only if the bi-directional connection all established, we start to handle event
            if num_established.get() >= 2 {
                // swarm_event::handle_connection_established(swarm, peer_id).await;
            }
        }
        SwarmEvent::ConnectionClosed {
            peer_id,
            connection_id,
            endpoint,
            num_established,
            ..
        } => {
            info!("[Connection closed] peer_id: {}, connection_id: {}, endpoint: {:?}, num_established: {:?}", peer_id, connection_id, endpoint, num_established);
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
