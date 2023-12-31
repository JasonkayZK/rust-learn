use std::time::Duration;

use libp2p::swarm::SwarmEvent;
use libp2p::{gossipsub, mdns};
use log::info;
use tokio::sync::mpsc;

use crate::behaviour::RecipeBehaviourEvent;
use crate::consts::{INIT_SYNC_STR, PEER_ID};
use crate::models::ListResponse;
use crate::swarm::handler::SwarmHandler;
use crate::sync::progress_manager::ProgressManager;

mod gossip_event;
pub mod handler;
mod mdns_event;

pub async fn handle_swarm_event(response_sender: mpsc::UnboundedSender<ListResponse>) {
    let event = SwarmHandler::select_next_some().await;
    info!("Income swarm Event: {:?}", event);

    match event {
        SwarmEvent::Behaviour(recipe_behaviours) => match recipe_behaviours {
            RecipeBehaviourEvent::Gossip(gossip_event) => match gossip_event {
                gossipsub::Event::Message {
                    propagation_source,
                    message,
                    ..
                } => {
                    gossip_event::handle_message(propagation_source, message, response_sender).await
                }
                gossipsub::Event::Subscribed { peer_id, topic } => {
                    let topic_id = topic.to_string();

                    if topic_id.eq(INIT_SYNC_STR) {
                        // For those new subscribers who is the new peers joined in the network, we should sync data
                        ProgressManager::init_sync_data(peer_id).await;
                    } else if topic_id.starts_with(&format!("sync-{}", *PEER_ID)) {
                        // When the peer subscribed the sync-peerId topic,
                        // we wait until received the INIT_SYNC message and get the progress
                        // Then start to send the sync data!
                        tokio::spawn(async move {
                            // Send the sync data to the initiate peer
                            let mut progress = 0;
                            for _ in 0..3 {
                                ProgressManager::list_keys().await;
                                // We've received the INIT_SYNC message, and update the progress, then sync the data!
                                if let Some(_sync_status) =
                                    ProgressManager::get_status(peer_id).await
                                {
                                    progress =
                                        ProgressManager::get_sync_progress(&peer_id.to_string())
                                            .await
                                            .unwrap()
                                            .unwrap_or_default()
                                            .get_first_checkpoint()
                                            .unwrap_or_default();
                                    info!(
                                        "Get synced progress success: {}, progress: {}",
                                        peer_id, progress
                                    );
                                    break;
                                }
                                tokio::time::sleep(Duration::from_secs(2)).await;
                            }
                            ProgressManager::send_sync_data(topic, progress).await;
                        });
                    }
                }
                gossipsub::Event::Unsubscribed { peer_id, topic } => {
                    let topic_id = topic.to_string();

                    if topic_id.eq(INIT_SYNC_STR) {
                        // For those subscribers who exit from the network, we should stop sync data
                        ProgressManager::stop_sync_data(peer_id).await;
                    }
                }
                gossipsub::Event::GossipsubNotSupported { .. } => {}
            },

            RecipeBehaviourEvent::Mdns(mdns_event) => match mdns_event {
                mdns::Event::Discovered(discovered_list) => {
                    mdns_event::handle_discovered(discovered_list).await
                }
                mdns::Event::Expired(expired_list) => {
                    mdns_event::handle_expired(expired_list).await
                }
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
