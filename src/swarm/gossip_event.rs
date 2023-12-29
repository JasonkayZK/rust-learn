use libp2p::gossipsub::Message;
use libp2p::PeerId;
use log::{error, info};
use tokio::sync::mpsc;

use crate::consts::PEER_ID;
use crate::models::{ListMode, ListRequest, ListResponse};
use crate::storage::read_local_recipes;

pub(crate) async fn handle_message(propagation_source: PeerId, msg: Message, response_sender: mpsc::UnboundedSender<ListResponse>) {
    if let Ok(resp) = serde_json::from_slice::<ListResponse>(&msg.data) {
        if resp.receiver == PEER_ID.to_string() {
            info!("Response from {}:", propagation_source);
            resp.data.iter().for_each(|r| info!("{:?}", r));
        }
    } else if let Ok(req) = serde_json::from_slice::<ListRequest>(&msg.data) {
        match req.mode {
            ListMode::All => {
                info!("Received ALL req: {:?} from {:?}", req, propagation_source);
                respond_with_public_recipes(
                    response_sender.clone(),
                    propagation_source.to_string(),
                );
            }
            ListMode::One(ref peer_id) => {
                if peer_id == &PEER_ID.to_string() {
                    info!("Received req: {:?} from {:?}", req, propagation_source);
                    respond_with_public_recipes(
                        response_sender.clone(),
                        propagation_source.to_string(),
                    );
                }
            }
        }
    }
}

fn respond_with_public_recipes(sender: mpsc::UnboundedSender<ListResponse>, receiver: String) {
    tokio::spawn(async move {
        match read_local_recipes().await {
            Ok(recipes) => {
                let resp = ListResponse {
                    mode: ListMode::All,
                    receiver,
                    data: recipes.into_iter().filter(|r| r.shared).collect(),
                };
                if let Err(e) = sender.send(resp) {
                    error!("error sending response via channel, {}", e);
                }
            }
            Err(e) => error!("error fetching local recipes to answer ALL request, {}", e),
        }
    });
}
