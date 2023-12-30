use std::collections::HashMap;

use tokio::fs;

use crate::dir::data_file;
use crate::models::Recipe;

pub async fn merge_recipes(new_recipes: HashMap<u64, Recipe>) -> anyhow::Result<HashMap<u64, Recipe>> {
    let mut data = read_local_recipes().await?;

    // Insert or update the recipe info
    for (id, recipe) in new_recipes {
        data.insert(id, recipe);
    }

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
