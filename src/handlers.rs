use std::collections::HashSet;

use anyhow::Result;
use libp2p::Swarm;
use log::{error, info};
use tokio::fs;
use tokio::sync::mpsc;

use crate::behaviour::RecipeBehaviour;
use crate::consts::{STORAGE_FILE_PATH, TOPIC};
use crate::models::{ListMode, ListRequest, ListResponse, Recipe};

pub async fn handle_list_peers(swarm: &mut Swarm<RecipeBehaviour>) {
    info!("Discovered Peers:");
    let nodes = swarm.behaviour().mdns.discovered_nodes();

    let mut unique_peers = HashSet::new();
    for peer in nodes {
        unique_peers.insert(peer);
    }
    unique_peers.iter().for_each(|p| info!("{}", p));
}

pub async fn handle_create_recipe(cmd: &str) {
    if let Some(rest) = cmd.strip_prefix("create r") {
        let elements: Vec<&str> = rest.split('|').collect();
        if elements.len() < 3 {
            info!("too few arguments - Format: name|ingredients|instructions");
        } else {
            let name = elements.first().expect("name is there");
            let ingredients = elements.get(1).expect("ingredients is there");
            let instructions = elements.get(2).expect("instructions is there");
            if let Err(e) = create_new_recipe(name, ingredients, instructions).await {
                error!("error creating recipe: {}", e);
            };
        }
    }
}

pub async fn handle_publish_recipe(cmd: &str) {
    if let Some(rest) = cmd.strip_prefix("publish r") {
        match rest.trim().parse::<usize>() {
            Ok(id) => {
                if let Err(e) = publish_recipe(id).await {
                    info!("error publishing recipe with id {}, {}", id, e)
                } else {
                    info!("Published Recipe with id: {}", id);
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
                .flood_sub
                .publish(TOPIC.clone(), json.as_bytes());
        }
        Some(recipes_peer_id) => {
            let req = ListRequest {
                mode: ListMode::One(recipes_peer_id.to_owned()),
            };
            let json = serde_json::to_string(&req).expect("can jsonify request");
            swarm
                .behaviour_mut()
                .flood_sub
                .publish(TOPIC.clone(), json.as_bytes());
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

async fn publish_recipe(id: usize) -> Result<()> {
    let mut local_recipes = read_local_recipes().await?;
    local_recipes
        .iter_mut()
        .filter(|r| r.id == id)
        .for_each(|r| r.shared = true);
    write_local_recipes(&local_recipes).await?;
    Ok(())
}

async fn create_new_recipe(name: &str, ingredients: &str, instructions: &str) -> Result<()> {
    let mut local_recipes = read_local_recipes().await?;
    let new_id = match local_recipes.iter().max_by_key(|r| r.id) {
        Some(v) => v.id + 1,
        None => 0,
    };
    local_recipes.push(Recipe {
        id: new_id,
        name: name.to_owned(),
        ingredients: ingredients.to_owned(),
        instructions: instructions.to_owned(),
        shared: false,
    });
    write_local_recipes(&local_recipes).await?;

    info!("Created recipe:");
    info!("Name: {}", name);
    info!("Ingredients: {}", ingredients);
    info!("Instructions:: {}", instructions);

    Ok(())
}

async fn write_local_recipes(recipes: &Vec<Recipe>) -> Result<()> {
    let json = serde_json::to_string(&recipes)?;
    fs::write(STORAGE_FILE_PATH, &json).await?;
    Ok(())
}

async fn read_local_recipes() -> Result<Vec<Recipe>> {
    let content = fs::read(STORAGE_FILE_PATH).await?;
    let result = serde_json::from_slice(&content)?;
    Ok(result)
}

pub fn respond_with_public_recipes(sender: mpsc::UnboundedSender<ListResponse>, receiver: String) {
    tokio::spawn(async move {
        match read_local_recipes().await {
            Ok(recipes) => {
                let resp = ListResponse {
                    mode: ListMode::All,
                    receiver,
                    data: recipes.into_iter().filter(|r| r.shared).collect(),
                };
                if let Err(e) = sender.send(resp) {
                    error!("error sending response via channel, {}", e);
                }
            }
            Err(e) => error!("error fetching local recipes to answer ALL request, {}", e),
        }
    });
}
