use libp2p::{PeerId, Swarm};
use crate::behaviour::RecipeBehaviour;

/// When connection established, data synchronization task will be launched
pub(crate) async fn handle_connection_established(swarm: &mut Swarm<RecipeBehaviour>, peer_id: PeerId) {

}
