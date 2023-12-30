use std::collections::HashMap;

use tokio::fs;

use crate::dir::data_file;
use crate::hlc::GlobalClock;
use crate::models::Recipe;
use crate::sync::models::OpEnum;

pub async fn merge_diff(logs: Vec<Option<OpEnum>>) -> anyhow::Result<Vec<u64>> {
    let mut data = read_local_recipes().await?;
    let mut query_ids = vec![];
    let mut delete_ids = vec![];

    // Step 1: Compute diff
    for x in logs {
        if let Some(op) = x {
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
    }

    // Step 2: Execute diff
    for id in delete_ids {
        data.remove(&id);
    }
    write_local_recipes(&data).await.unwrap();

    Ok(query_ids)
}

pub async fn merge_recipes(new_recipes: HashMap<u64, Recipe>) -> anyhow::Result<HashMap<u64, Recipe>> {
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
