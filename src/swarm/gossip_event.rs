use std::str::FromStr;

use chrono::Local;
use libp2p::{PeerId, Swarm};
use libp2p::gossipsub::{IdentTopic, Message};
use log::{error, info};
use tokio::sync::mpsc;

use crate::behaviour::RecipeBehaviour;
use crate::consts::{INIT_SYNC_STR, PEER_ID, RECIPE_TOPIC};
use crate::models::{InitSyncMessage, ListMode, ListRequest, ListResponse, SyncDataRequest, SyncDataResponse, SyncLogData};
use crate::storage::{merge_recipes, read_local_recipes};
use crate::sync::progress_manager::{ProgressManager, SyncStatus};

pub(crate) async fn handle_message(swarm: &mut Swarm<RecipeBehaviour>, propagation_source: PeerId, msg: Message, response_sender: mpsc::UnboundedSender<ListResponse>) {
    let topic_id = msg.topic.to_string();

    if topic_id.eq(&RECIPE_TOPIC.to_string()) {
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
        } else {
            error!("Unable to serialize message: {:?} from type: {}", msg, &RECIPE_TOPIC.to_string());
        }
    } else if topic_id.eq(INIT_SYNC_STR) {
        // We received the INIT_SYNC message, this means that we are the new joined peer
        if let Ok(init_sync_msg) = serde_json::from_slice::<InitSyncMessage>(&msg.data) {
            // Step 1: Subscribe to the `sync-old-new` topic to receive sync data from old
            let gossip = &mut swarm.behaviour_mut().gossip;

            let topic_name = ProgressManager::get_sync_topic_id(&init_sync_msg.old_peer.to_string(), &init_sync_msg.new_peer.to_string());
            let topic = IdentTopic::new(topic_name);
            gossip.subscribe(&topic).unwrap();

            // Step 2: Add Status to the ProgressManager
            ProgressManager::set_status(PeerId::from_str(&init_sync_msg.old_peer).unwrap(), SyncStatus::Start(Local::now().timestamp_millis())).await;

            // Step 3: Send own progress to let other peers know the sync status, and begin to receive the sync data from old
            let mut new_init_sync_msg = init_sync_msg.clone();
            new_init_sync_msg.progress = ProgressManager::get_key(&init_sync_msg.old_peer).await.unwrap().unwrap_or_default();
            let json = serde_json::to_string(&new_init_sync_msg).expect("can jsonify SyncMessage request");
            gossip.publish(topic.clone(), json).unwrap();

            // Step 4: Meanwhile, send the sync data to the old peer
            let progress = ProgressManager::get_key(&init_sync_msg.old_peer).await.unwrap().unwrap_or_default();
            ProgressManager::send_sync_data(swarm, topic, progress).await;
        } else {
            error!("Unable to serialize message: {:?} from type: {}", msg, INIT_SYNC_STR);
        }
    } else if topic_id.starts_with("sync") {
        // If we received the InitSyncMessage in the `sync-old-new` topic, which means that the new peer joined into the topic
        //  and is announcing its sync progress
        if let Ok(sync_message) = serde_json::from_slice::<InitSyncMessage>(&msg.data) {
            // Received a new sync request sent from new server
            if sync_message.old_peer.eq(&PEER_ID.to_string()) {
                // Subscribe to the corresponding topic, and begin to send the un-synced data
                let sync_topic_id = ProgressManager::get_sync_topic_id(&sync_message.old_peer, &sync_message.new_peer);
                let sync_topic = IdentTopic::new(sync_topic_id.clone());
                // Subscribe the topic to receive the old server sent data
                swarm.behaviour_mut().gossip.subscribe(&sync_topic).unwrap();
            }
        } else if let Ok(sync_log_message) = serde_json::from_slice::<SyncLogData>(&msg.data) {
            // When received SyncLogData message, we compute the whole log, and then query for the data
        } else if let Ok(sync_data_req) = serde_json::from_slice::<SyncDataRequest>(&msg.data) {
            // When received SyncDataRequest, we send all data correspond to the ids, then update sync table and remove the status

            // Step 1: Send response data
            let mut all_data = read_local_recipes().await.unwrap();
            all_data.retain(|x, _| sync_data_req.recipe_ids.contains(&x));
            let resp = SyncDataResponse {
                recipes: all_data,
                progress_idx: sync_data_req.progress_idx,
            };
            let json = serde_json::to_string(&resp).expect("can jsonify request");
            swarm.behaviour_mut().gossip.publish(msg.topic.clone(), json).unwrap();

            // Step 2: Update sync progress table
            ProgressManager::set_key(&propagation_source.to_string(), sync_data_req.progress_idx).await.unwrap();

            // Step 3: Remove sync status
            ProgressManager::remove_status(propagation_source).await;
        } else if let Ok(sync_data_resp) = serde_json::from_slice::<SyncDataResponse>(&msg.data) {
            // When received SyncDataResponse, we just merge the data, then update sync table and remove the status

            // Step 1: Merge data
            merge_recipes(sync_data_resp.recipes).await.unwrap();

            // Step 2: Update sync progress table
            ProgressManager::set_key(&propagation_source.to_string(), sync_data_resp.progress_idx).await.unwrap();

            // Step 3: Remove sync status
            ProgressManager::remove_status(propagation_source).await;
        }
    } else {
        error!("Unable to serialize message: {:?} from type: {}", msg, topic_id);
    }
}

fn respond_with_public_recipes(sender: mpsc::UnboundedSender<ListResponse>, receiver: String) {
    tokio::spawn(async move {
        match read_local_recipes().await {
            Ok(recipes) => {
                let resp = ListResponse {
                    mode: ListMode::All,
                    receiver,
                    data: recipes.into_iter().filter(|(_, r)| r.shared).collect(),
                };
                if let Err(e) = sender.send(resp) {
                    error!("error sending response via channel, {}", e);
                }
            }
            Err(e) => error!("error fetching local recipes to answer ALL request, {}", e),
        }
    });
}
