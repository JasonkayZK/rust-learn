use std::collections::HashMap;
use std::sync::OnceLock;

use chrono::Local;
use libp2p::{PeerId, Swarm};
use libp2p::futures::executor::block_on;
use libp2p::gossipsub::IdentTopic;
use log::{info, warn};
use redb::{Database, Error, ReadableTable, TableDefinition};
use tokio::sync::Mutex;

use crate::behaviour::RecipeBehaviour;
use crate::consts::{INIT_SYNC_TOPIC, PEER_ID};
use crate::dir::sync_log_file;
use crate::models::InitSyncMessage;
use crate::sync::oplog::OpLogHandler;

const TABLE: TableDefinition<&str, u64> = TableDefinition::new("sync_progress");

static PROGRESS_MANAGER: OnceLock<Mutex<ProgressManager>> = OnceLock::new();

#[derive(Debug, Clone)]
pub enum SyncStatus {
    // Start timestamp
    Start(i64),

    // Finished status will not be used, since the entry will be removed when finished
    // Finished,
}

pub struct ProgressManager {
    db: Database,
    status_table: HashMap<PeerId, SyncStatus>,
}

impl ProgressManager {
    async fn global() -> &'static Mutex<Self> {
        PROGRESS_MANAGER.get_or_init(|| {
            block_on(async {
                let db = Database::create(sync_log_file()).unwrap();
                Mutex::new(Self {
                    db,
                    status_table: HashMap::new(),
                })
            })
        })
    }

    pub async fn remove_status(peer_id: PeerId) {
        let table = &mut Self::global().await.lock().await.status_table;
        table.remove(&peer_id);
    }

    pub async fn set_status(peer_id: PeerId, status: SyncStatus) {
        let table = &mut Self::global().await.lock().await.status_table;
        table.insert(peer_id, status);
    }

    pub async fn get_status(peer_id: &PeerId) -> Option<SyncStatus> {
        let table = &Self::global().await.lock().await.status_table;
        table.get(peer_id).cloned()
    }

    pub async fn get_key(k: &str) -> Result<Option<u64>, Error> {
        let db = &mut Self::global().await.lock().await.db;
        let read_txn = db.begin_read()?;
        let table = read_txn.open_table(TABLE)?;
        let ret = table.get(k)?.map(|val| val.value());
        Ok(ret)
    }

    pub async fn set_key(k: &str, v: u64) -> Result<(), Error> {
        let db = &mut Self::global().await.lock().await.db;
        let write_txn = db.begin_write()?;
        {
            let mut table = write_txn.open_table(TABLE)?;
            table.insert(k, v)?;
        }
        write_txn.commit()?;

        Ok(())
    }

    pub async fn init_sync_data(swarm: &mut Swarm<RecipeBehaviour>, new_peer_id: PeerId) {
        let progress = Self::get_key(&new_peer_id.to_string()).await.unwrap().unwrap_or_default();
        let mut manager = Self::global().await.lock().await;

        if let Some(status) = manager.status_table.get(&new_peer_id) {
            warn!("Their has already been a sync progress: {:?}", status);
            return;
        }

        // Step 1: Create and subscribe a new topic for the two peers: sync-old-new
        // Once the sync task done, the two peers will unsubscribe the topic
        let sync_topic_id = Self::get_sync_topic_id(&PEER_ID.to_string(), &new_peer_id.to_string());
        let sync_topic = IdentTopic::new(sync_topic_id.clone());
        let gossip = &mut swarm.behaviour_mut().gossip;
        gossip.subscribe(&sync_topic).unwrap();

        // Step 2: Add sync status to the table
        manager.status_table.insert(new_peer_id, SyncStatus::Start(Local::now().timestamp_millis()));

        // Step 3: Send sync message to the new peer
        let req = InitSyncMessage {
            progress,
            old_peer: PEER_ID.to_string(),
            new_peer: new_peer_id.to_string(),
        };
        let json = serde_json::to_string(&req).expect("can jsonify SyncMessage request");
        gossip.publish(INIT_SYNC_TOPIC.clone(), json.as_bytes()).unwrap();
    }

    /// For peer node exit the network, that will unsubscribe the init-sync topic
    pub async fn stop_sync_data(swarm: &mut Swarm<RecipeBehaviour>, new_peer_id: PeerId) {
        let mut manager = Self::global().await.lock().await;

        match manager.status_table.get(&new_peer_id) {
            // No sync task yet
            None => {
                info!("Data sync has finished, exit sync data");
            }
            // Sync task is undergoing!
            Some(status) => {
                warn!("Data sync is undergoing: {:?}, now stop!", status);

                // Step 1: Unsubscribe the sync topic
                let sync_topic_id = Self::get_sync_topic_id(&PEER_ID.to_string(), &new_peer_id.to_string());
                let sync_topic = IdentTopic::new(sync_topic_id.clone());
                let gossip = &mut swarm.behaviour_mut().gossip;
                gossip.subscribe(&sync_topic).unwrap();

                swarm.behaviour_mut().gossip.unsubscribe(&sync_topic).unwrap();

                // Step 2: Remove sync status entry in the table
                manager.status_table.remove(&new_peer_id);
            }
        }
    }

    pub async fn send_sync_data(swarm: &mut Swarm<RecipeBehaviour>, topic: IdentTopic, progress_start_idx: u64) {
        let snapshot_progress_idx = OpLogHandler::get_info().await.length;
        let range: Vec<u64> = (progress_start_idx..snapshot_progress_idx).collect();
        let logs = OpLogHandler::get_batch(range.as_slice()).await.unwrap();
        let json = serde_json::to_string(&logs).expect("can jsonify send_sync_data message");
        swarm.behaviour_mut().gossip.publish(topic, json).unwrap();
    }

    pub fn get_sync_topic_id(old_peer_id: &str, new_peer_id: &str) -> String {
        format!("sync-{}-{}", old_peer_id, new_peer_id)
    }
}
