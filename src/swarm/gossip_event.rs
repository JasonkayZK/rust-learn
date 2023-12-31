use std::convert::TryFrom;
use std::str::FromStr;

use chrono::Local;
use libp2p::gossipsub::Message;
use libp2p::PeerId;
use log::{error, info, warn};
use tokio::sync::mpsc;

use crate::consts::{INIT_SYNC_STR, PEER_ID, RECIPE_TOPIC};
use crate::models::{
    InitSyncMessage, ListMode, ListRequest, ListResponse, SyncDataRequest, SyncDataResponse,
    SyncLogData,
};
use crate::storage::{merge_diff, merge_recipes, read_local_recipes};
use crate::swarm::handler::SwarmHandler;
use crate::sync::models::OpEnum;
use crate::sync::progress_manager::{ProgressManager, SyncStatus};

pub(crate) async fn handle_message(
    propagation_source: PeerId,
    msg: Message,
    response_sender: mpsc::UnboundedSender<ListResponse>,
) {
    let topic_id = msg.topic.to_string();
    warn!("Got swarm message: {:?}", msg);

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
            error!(
                "Unable to serialize message: {:?} from type: {}",
                msg,
                &RECIPE_TOPIC.to_string()
            );
        }
    } else if topic_id.eq(INIT_SYNC_STR) {
        // We received the INIT_SYNC message, this means that we are the follow peer
        if let Ok(init_sync_msg) = serde_json::from_slice::<InitSyncMessage>(&msg.data) {
            // Step 1: Subscribe to the `sync-follow-initiate` topic to listen sync data from initiate here!
            let (send_sync_topic, receive_sync_topic) = ProgressManager::get_sync_topics(
                &init_sync_msg.follow_peer.to_string(),
                &init_sync_msg.initiate_peer.to_string(),
            );
            SwarmHandler::subscribe(&send_sync_topic).await.unwrap();
            SwarmHandler::subscribe(&receive_sync_topic).await.unwrap();

            // Step 2: Add Status to the ProgressManager
            // This will trigger the sync-initiate-follow topic to start send sync data!
            ProgressManager::set_status(
                PeerId::from_str(&init_sync_msg.initiate_peer).unwrap(),
                SyncStatus::Start(Local::now().timestamp_millis()),
            )
            .await;

            // Step 3: Save peer sync progress
            ProgressManager::set_key(&propagation_source.to_string(), init_sync_msg.progress)
                .await
                .unwrap();
        } else {
            error!(
                "Unable to serialize message: {:?} from type: {}",
                msg, INIT_SYNC_STR
            );
        }
    } else if topic_id.starts_with("sync-") {
        // If we received the InitSyncMessage in the `sync-old-new` topic, which means that the new peer joined into the topic
        //  and is announcing its sync progress
        // if let Ok(sync_message) = serde_json::from_slice::<InitSyncMessage>(&msg.data) {
        //     // Received a new sync request sent from new server
        //     if sync_message.initiate_peer.eq(&PEER_ID.to_string()) {
        //         // Subscribe to the corresponding topic, and begin to send the un-synced data
        //         let sync_topic_id = ProgressManager::get_sync_topic_id(&sync_message.initiate_peer, &sync_message.follow_peer);
        //         let sync_topic = IdentTopic::new(sync_topic_id.clone());
        //         // Subscribe the topic to receive the old server sent data
        //         SwarmHandler::subscribe(&sync_topic).await.unwrap();
        //     }
        // } else if let Ok(sync_log_message) = serde_json::from_slice::<SyncLogData>(&msg.data) {
        if let Ok(sync_log_message) = serde_json::from_slice::<SyncLogData>(&msg.data) {
            // When received SyncLogData message, we compute the whole log, and then query for the data

            // Step 1: Compute diff
            let logs: Vec<Option<OpEnum>> = sync_log_message
                .logs
                .into_iter()
                .map(|item| item.map(|x| OpEnum::try_from(x).unwrap()))
                .collect();
            let query_ids = merge_diff(logs).await.unwrap();

            // Step 2: Send sync data request
            let req = SyncDataRequest {
                recipe_ids: query_ids,
                progress_idx: sync_log_message.progress_idx,
            };
            let json = serde_json::to_string(&req).expect("can jsonify request");
            SwarmHandler::publish(msg.topic.clone(), json)
                .await
                .unwrap();
        } else if let Ok(sync_data_req) = serde_json::from_slice::<SyncDataRequest>(&msg.data) {
            // When received SyncDataRequest, we send all data correspond to the ids, then update sync table and remove the status

            // Step 1: Send response data
            let mut all_data = read_local_recipes().await.unwrap();
            all_data.retain(|x, _| sync_data_req.recipe_ids.contains(x));
            let resp = SyncDataResponse {
                recipes: all_data,
                progress_idx: sync_data_req.progress_idx,
            };
            let json = serde_json::to_string(&resp).expect("can jsonify request");
            SwarmHandler::publish(msg.topic.clone(), json)
                .await
                .unwrap();

            // Step 2: Update sync progress table
            ProgressManager::set_key(&propagation_source.to_string(), sync_data_req.progress_idx)
                .await
                .unwrap();

            // Step 3: Remove sync status
            ProgressManager::remove_status(propagation_source).await;
        } else if let Ok(sync_data_resp) = serde_json::from_slice::<SyncDataResponse>(&msg.data) {
            // When received SyncDataResponse, we just merge the data, then update sync table and remove the status

            // Step 1: Merge data
            merge_recipes(sync_data_resp.recipes).await.unwrap();

            // Step 2: Update sync progress table
            ProgressManager::set_key(&propagation_source.to_string(), sync_data_resp.progress_idx)
                .await
                .unwrap();

            // Step 3: Remove sync status
            ProgressManager::remove_status(propagation_source).await;
        }
    } else {
        error!(
            "Unable to serialize message: {:?} from type: {}",
            msg, topic_id
        );
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
