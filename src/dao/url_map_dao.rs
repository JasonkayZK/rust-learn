use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, FromRow, Clone, Serialize, Deserialize)]
pub struct UrlMap {
    key: String,
    url: String,
}
