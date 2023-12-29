use libp2p::{Multiaddr, PeerId, Swarm};

use crate::behaviour::RecipeBehaviour;

pub(crate) async fn handle_discovered(swarm: &mut Swarm<RecipeBehaviour>, discovered_list: Vec<(PeerId, Multiaddr)>) {
    let behavior_mut = swarm.behaviour_mut();
    for (peer, _addr) in discovered_list {
        behavior_mut.gossip.add_explicit_peer(&peer);
    }
}

pub(crate) async fn handle_expired(swarm: &mut Swarm<RecipeBehaviour>, expired_list: Vec<(PeerId, Multiaddr)>) {
    let behavior_mut = swarm.behaviour_mut();
    for (peer, _addr) in expired_list {
        if !behavior_mut.mdns.has_node(&peer) {
            behavior_mut.gossip.remove_explicit_peer(&peer);
        }
    }
}
