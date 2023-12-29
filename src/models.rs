use std::hash::Hash;

use serde::{Deserialize, Serialize};

/// The recipe data for cook
#[derive(Debug, Hash, Clone, Serialize, Deserialize)]
pub struct Recipe {
    pub id: usize,
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
    pub data: Vec<Recipe>,
    pub receiver: String,
}

pub enum EventType {
    Response(ListResponse),
    Input(String),
}
