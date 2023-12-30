use std::collections::HashMap;
use std::hash::Hash;

use serde::{Deserialize, Serialize};

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

/// Old server request sync action actively to the new server
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InitSyncMessage {
    pub progress: u64,
    pub old_peer: String,
    pub new_peer: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SyncLogData {
    logs: Vec<Option<Vec<u8>>>,
    pub progress_idx: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SyncDataRequest {
    pub recipe_ids: Vec<u64>,
    pub progress_idx: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SyncDataResponse {
    pub recipes: HashMap<u64, Recipe>,
    pub progress_idx: u64,
}

pub enum EventType {
    Response(ListResponse),
    Input(String),
}
