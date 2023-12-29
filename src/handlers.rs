use std::collections::hash_map::DefaultHasher;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};

use anyhow::Result;
use libp2p::Swarm;
use log::{error, info};

use crate::behaviour::RecipeBehaviour;
use crate::consts::TOPIC;
use crate::models::{ListMode, ListRequest, Recipe};
use crate::storage::{read_local_recipes, write_local_recipes};
use crate::sync::models::OpEnum;
use crate::sync::oplog::OpLogHandler;

/// List all peers in P2P network
///
/// Command: `ls p`
pub async fn handle_list_peers(swarm: &mut Swarm<RecipeBehaviour>) {
    info!("Discovered Peers:");
    let nodes = swarm.behaviour().mdns.discovered_nodes();

    let mut unique_peers = HashSet::new();
    for peer in nodes {
        unique_peers.insert(peer);
    }
    unique_peers.iter().for_each(|p| info!("{}", p));
}

/// Create recipe
///
/// Command: `create r name|recipe_ingredients|recipe_instruction`
pub async fn handle_create_recipe(cmd: &str) {
    if let Some(rest) = cmd.strip_prefix("create r") {
        let elements: Vec<&str> = rest.split('|').collect();
        if elements.len() < 3 {
            info!("too few arguments - Format: name|ingredients|instructions");
            return;
        }

        // Step 1: Save element
        let name = elements.first().expect("name is there");
        let ingredients = elements.get(1).expect("ingredients is there");
        let instructions = elements.get(2).expect("instructions is there");
        match create_new_recipe(name, ingredients, instructions).await {
            Ok(recipe) => {

                // Step 2: Write Log:
                let mut hasher = DefaultHasher::new();
                recipe.hash(&mut hasher);
                let hash_value = hasher.finish();
                OpLogHandler::append(OpEnum::Insert(hash_value.to_string()).to_string().as_bytes()).await.unwrap();
                info!("Recipe create log appended: {}", hash_value);
            }
            Err(e) => {
                error!("error creating recipe: {}", e);
            }
        }
    }
}

pub async fn handle_delete_recipe(cmd: &str) {
    if let Some(rest) = cmd.strip_prefix("delete r") {
        match rest.trim().parse::<usize>() {
            Ok(id) => {
                // Step 1: Delete element
                match delete_recipes(id).await {
                    Ok(recipe) => {
                        if let Some(recipe) = recipe {
                            info!("Deleted Recipe with id: {}", recipe.id);

                            // Step 2: Write Log:
                            let mut hasher = DefaultHasher::new();
                            recipe.hash(&mut hasher);
                            let hash_value = hasher.finish();
                            OpLogHandler::append(OpEnum::Delete(hash_value.to_string()).to_string().as_bytes()).await.unwrap();
                            info!("Recipe delete log appended: {}", hash_value);
                        }
                    }
                    Err(e) => {
                        error!("error delete recipe with id {}, {}", id, e)
                    }
                }
            }
            Err(e) => error!("invalid id: {}, {}", rest.trim(), e),
        };
    }
}

pub async fn handle_publish_recipe(cmd: &str) {
    if let Some(rest) = cmd.strip_prefix("publish r") {
        match rest.trim().parse::<usize>() {
            Ok(id) => {
                // Step 1: Update element
                match publish_recipe(id).await {
                    Ok(recipes) => {
                        if let Some((old_recipe, new_recipe)) = recipes {
                            info!("Published Recipe with id: {}", id);

                            // Step 2: Write Log:
                            let mut hasher = DefaultHasher::new();
                            old_recipe.hash(&mut hasher);
                            let old_hash_value = hasher.finish().to_string();
                            let mut hasher = DefaultHasher::new();
                            new_recipe.hash(&mut hasher);
                            let new_hash_value = hasher.finish().to_string();
                            info!("Recipe update log appended: {}->{}", old_hash_value, new_hash_value);
                            OpLogHandler::append(OpEnum::Update(old_hash_value, new_hash_value).to_string().as_bytes()).await.unwrap();
                        }
                    }
                    Err(e) => {
                        error!("error publishing recipe with id {}, {}", id, e)
                    }
                }
            }
            Err(e) => error!("invalid id: {}, {}", rest.trim(), e),
        };
    }
}

pub async fn handle_list_recipes(cmd: &str, swarm: &mut Swarm<RecipeBehaviour>) {
    let rest = cmd.strip_prefix("ls r ");
    match rest {
        Some("all") => {
            let req = ListRequest {
                mode: ListMode::All,
            };
            let json = serde_json::to_string(&req).expect("can jsonify request");
            swarm
                .behaviour_mut()
                .gossip
                .publish(TOPIC.clone(), json.as_bytes()).unwrap();
        }
        Some(recipes_peer_id) => {
            let req = ListRequest {
                mode: ListMode::One(recipes_peer_id.to_owned()),
            };
            let json = serde_json::to_string(&req).expect("can jsonify request");
            swarm
                .behaviour_mut()
                .gossip
                .publish(TOPIC.clone(), json.as_bytes()).unwrap();
        }
        None => {
            match read_local_recipes().await {
                Ok(v) => {
                    info!("Local Recipes ({})", v.len());
                    v.iter().for_each(|r| info!("{:?}", r));
                }
                Err(e) => error!("error fetching local recipes: {}", e),
            };
        }
    };
}

async fn publish_recipe(id: usize) -> Result<Option<(Recipe, Recipe)>> {
    let mut local_recipes = read_local_recipes().await?;
    let mut ret_recipes = None;
    for x in local_recipes.iter_mut() {
        if x.id == id {
            let origin_recipe = x.clone();
            x.shared = true;
            let new_recipe = x.clone();
            ret_recipes = Some((origin_recipe, new_recipe));
            break;
        }
    }

    write_local_recipes(&local_recipes).await?;
    Ok(ret_recipes)
}

async fn delete_recipes(id: usize) -> Result<Option<Recipe>> {
    let mut local_recipes = read_local_recipes().await?;

    let mut ret = None;
    for x in local_recipes.iter() {
        if x.id == id {
            ret = Some(x.clone());
            break;
        }
    }

    local_recipes.retain(|r| r.id != id);
    write_local_recipes(&local_recipes).await?;
    Ok(ret)
}

async fn create_new_recipe(name: &str, ingredients: &str, instructions: &str) -> Result<Recipe> {
    let mut local_recipes = read_local_recipes().await?;
    let new_id = match local_recipes.iter().max_by_key(|r| r.id) {
        Some(v) => v.id + 1,
        None => 0,
    };

    let recipe = Recipe {
        id: new_id,
        name: name.to_owned(),
        ingredients: ingredients.to_owned(),
        instructions: instructions.to_owned(),
        shared: false,
    };
    local_recipes.push(recipe.clone());
    write_local_recipes(&local_recipes).await?;

    info!("Created recipe:");
    info!("Name: {}", name);
    info!("Ingredients: {}", ingredients);
    info!("Instructions:: {}", instructions);

    Ok(recipe)
}
