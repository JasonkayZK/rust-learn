use std::collections::HashMap;
use std::hash::Hash;

use serde::{Deserialize, Serialize};

use crate::sync::progress::SyncEnum;

/// The recipe data for cook
#[derive(Debug, Hash, Clone, Serialize, Deserialize)]
pub struct Recipe {
    pub id: u64,
    pub name: String,
    pub ingredients: String,
    pub instructions: String,
    pub shared: bool,
}

/// Fetch data mode
#[derive(Debug, Serialize, Deserialize)]
pub enum ListMode {
    /// Fetch from all peers
    All,

    /// Fetch from one specific peer
    One(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListRequest {
    pub mode: ListMode,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListResponse {
    pub mode: ListMode,
    pub data: HashMap<u64, Recipe>,
    pub receiver: String,
}

/// Initiate server request sync data from the follow server
///
/// This will generate two topics:
///  - sync-{initiate_peer}-{follow_peer}: initiate_peer send sync data to follow_peer
///  - sync-{follow_peer}-{initiate_peer}: follow_peer send sync data to initiate_peer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InitSyncMessage {
    pub start_checkpoint_idx: u64,
    pub initiate_peer: String,
    pub follow_peer: String,
}

/// The synced logs data
#[derive(Debug, Serialize, Deserialize)]
pub struct SyncLogData {
    pub logs: Vec<Option<Vec<u8>>>,
    pub progress_indexes: SyncEnum,
    pub total_log_cnt: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SyncDataRequest {
    pub recipe_ids: Vec<u64>,
    pub progress_indexes: SyncEnum,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SyncDataResponse {
    pub recipes: HashMap<u64, Recipe>,
    pub progress_indexes: SyncEnum,
}

pub enum EventType {
    Response(ListResponse),
    Input(String),
}
