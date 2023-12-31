use libp2p::{Multiaddr, PeerId};

use crate::swarm::handler::SwarmHandler;

pub(crate) async fn handle_discovered(discovered_list: Vec<(PeerId, Multiaddr)>) {
    for (peer, _addr) in discovered_list {
        SwarmHandler::add_explicit_peer(&peer).await;
    }
}

pub(crate) async fn handle_expired(expired_list: Vec<(PeerId, Multiaddr)>) {
    for (peer, _addr) in expired_list {
        if !SwarmHandler::has_node(&peer).await {
            SwarmHandler::remove_explicit_peer(&peer).await;
        }
    }
}
