use std::collections::HashMap;

use log::warn;
use tokio::fs;

use crate::dir::data_file;
use crate::hlc::GlobalClock;
use crate::models::{BroadcastOptMessage, Recipe};
use crate::sync::models::OpEnum;
use crate::sync::progress::SyncEnum;
use crate::sync::progress_manager::ProgressManager;

pub async fn apply_opt(
    peer_id: &str,
    broadcast_opt_message: BroadcastOptMessage,
) -> anyhow::Result<()> {
    // Step 1: Apply the opt
    let mut data = read_local_recipes().await?;
    match broadcast_opt_message.opt {
        OpEnum::Insert(item_id, _timestamp) => {
            match data.get(&item_id) {
                None => {
                    data.insert(item_id, broadcast_opt_message.data.unwrap());
                }
                Some(current_item) => {
                    warn!("Filter duplicate insert!");
                    return Ok(());
                }
            }
        }
        OpEnum::Update(old_item_id, new_item_id, timestamp) => {
            // Here we operate delete after the peer's update, just leave it as deleted!
            if let Some(current_item) = data.get(&old_item_id) {
                if current_item.opt_timestamp.gt(new_item_id) {}
            } else if let Some(current_item) = data.get(&new_item_id) {

            } else {
                // We got no logs about the current log!
                data.remove(&old_item_id);
                data.insert(new_item_id, broadcast_opt_message.data.unwrap());
            }
        }
        OpEnum::Delete(item_id, timestamp) => {
            data.remove(&item_id);
        }
    }
    write_local_recipes(&data).await.unwrap();

    // Step 2: update sync progress
    ProgressManager::set_sync_progress(peer_id, SyncEnum::Single(broadcast_opt_message.log_idx))
        .await
        .unwrap();

    Ok(())
}

pub async fn merge_diff(logs: Vec<Option<OpEnum>>) -> anyhow::Result<Vec<u64>> {
    let mut data = read_local_recipes().await?;
    let mut query_ids = vec![];
    let mut delete_ids = vec![];

    // Step 1: Compute diff
    for op in logs.into_iter().flatten() {
        match op {
            OpEnum::Insert(recipe_id, timestamp) => {
                GlobalClock::update_with_timestamp(&timestamp).await;
                if data.contains_key(&recipe_id) {
                    continue;
                } else {
                    query_ids.push(recipe_id);
                }
            }
            OpEnum::Update(old_id, new_id, timestamp) => {
                GlobalClock::update_with_timestamp(&timestamp).await;
                if data.contains_key(&new_id) {
                    continue;
                } else {
                    query_ids.push(new_id);
                    delete_ids.push(old_id);
                }
            }
            OpEnum::Delete(recipe_id, timestamp) => {
                GlobalClock::update_with_timestamp(&timestamp).await;
                if !data.contains_key(&recipe_id) {
                    continue;
                } else {
                    delete_ids.push(recipe_id);
                }
            }
        }
    }

    // Step 2: Execute diff
    for id in delete_ids {
        data.remove(&id);
    }
    write_local_recipes(&data).await.unwrap();

    Ok(query_ids)
}

pub async fn merge_recipes(
    new_recipes: HashMap<u64, Recipe>,
) -> anyhow::Result<HashMap<u64, Recipe>> {
    let mut data = read_local_recipes().await?;

    // Insert or update the recipe info
    for (id, recipe) in new_recipes {
        data.insert(id, recipe);
    }

    write_local_recipes(&data).await.unwrap();

    Ok(data)
}

pub async fn write_local_recipes(recipes: &HashMap<u64, Recipe>) -> anyhow::Result<()> {
    let json = serde_json::to_string(&recipes)?;
    fs::write(data_file(), &json).await?;
    Ok(())
}

pub async fn read_local_recipes() -> anyhow::Result<HashMap<u64, Recipe>> {
    let content = fs::read(data_file()).await?;
    let result = serde_json::from_slice(&content)?;
    Ok(result)
}
