use tokio::fs;
use crate::dir::data_file;
use crate::models::Recipe;

pub async fn write_local_recipes(recipes: &Vec<Recipe>) -> anyhow::Result<()> {
    let json = serde_json::to_string(&recipes)?;
    fs::write(data_file(), &json).await?;
    Ok(())
}

pub async fn read_local_recipes() -> anyhow::Result<Vec<Recipe>> {
    let content = fs::read(data_file()).await?;
    let result = serde_json::from_slice(&content)?;
    Ok(result)
}
