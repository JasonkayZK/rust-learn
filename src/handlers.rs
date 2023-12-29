use std::collections::HashSet;

use anyhow::Result;
use libp2p::floodsub::FloodsubEvent;
use libp2p::futures::StreamExt;
use libp2p::mdns::Event;
use libp2p::Swarm;
use libp2p::swarm::SwarmEvent;
use log::{debug, error, info};
use tokio::fs;
use tokio::sync::mpsc;

use crate::behaviour::{RecipeBehaviour, RecipeBehaviourEvent};
use crate::consts::{PEER_ID, TOPIC};
use crate::dir::data_file;
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
            return;
        }

        let name = elements.first().expect("name is there");
        let ingredients = elements.get(1).expect("ingredients is there");
        let instructions = elements.get(2).expect("instructions is there");
        if let Err(e) = create_new_recipe(name, ingredients, instructions).await {
            error!("error creating recipe: {}", e);
        };
    }
}

pub async fn handle_delete_recipe(cmd: &str) {
    if let Some(rest) = cmd.strip_prefix("delete r") {
        match rest.trim().parse::<usize>() {
            Ok(id) => {
                if let Err(e) = delete_recipes(id).await {
                    info!("error delete recipe with id {}, {}", id, e)
                } else {
                    info!("Deleted Recipe with id: {}", id);
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

pub async fn handle_swarm_event(
    response_sender: mpsc::UnboundedSender<ListResponse>,
    swarm: &mut Swarm<RecipeBehaviour>,
) {
    let event = swarm.select_next_some().await;
    info!("Income swarm Event: {:?}", event);

    match event {
        SwarmEvent::Behaviour(recipe_behaviours) => match recipe_behaviours {
            RecipeBehaviourEvent::Floodsub(flood_sub_event) => match flood_sub_event {
                FloodsubEvent::Message(msg) => {
                    if let Ok(resp) = serde_json::from_slice::<ListResponse>(&msg.data) {
                        if resp.receiver == PEER_ID.to_string() {
                            info!("Response from {}:", msg.source);
                            resp.data.iter().for_each(|r| info!("{:?}", r));
                        }
                    } else if let Ok(req) = serde_json::from_slice::<ListRequest>(&msg.data) {
                        match req.mode {
                            ListMode::All => {
                                info!("Received ALL req: {:?} from {:?}", req, msg.source);
                                respond_with_public_recipes(
                                    response_sender.clone(),
                                    msg.source.to_string(),
                                );
                            }
                            ListMode::One(ref peer_id) => {
                                if peer_id == &PEER_ID.to_string() {
                                    info!("Received req: {:?} from {:?}", req, msg.source);
                                    respond_with_public_recipes(
                                        response_sender.clone(),
                                        msg.source.to_string(),
                                    );
                                }
                            }
                        }
                    }
                }
                FloodsubEvent::Subscribed { .. } => {}
                FloodsubEvent::Unsubscribed { .. } => {}
            },
            RecipeBehaviourEvent::Mdns(mdns_event) => match mdns_event {
                Event::Discovered(discovered_list) => {
                    let behavior_mut = swarm.behaviour_mut();
                    for (peer, _addr) in discovered_list {
                        behavior_mut.flood_sub.add_node_to_partial_view(peer);
                    }
                }
                Event::Expired(expired_list) => {
                    let behavior_mut = swarm.behaviour_mut();
                    for (peer, _addr) in expired_list {
                        if !behavior_mut.mdns.has_node(&peer) {
                            behavior_mut.flood_sub.remove_node_from_partial_view(&peer);
                        }
                    }
                }
            },
        },
        SwarmEvent::ConnectionEstablished {
            peer_id,
            connection_id,
            endpoint,
            num_established,
            ..
        } => {
            debug!("[Connection established] peer_id: {}, connection_id: {}, endpoint: {:?}, num_established: {:?}", peer_id, connection_id, endpoint, num_established);
        }
        SwarmEvent::ConnectionClosed {
            peer_id,
            connection_id,
            endpoint,
            num_established,
            ..
        } => {
            debug!("[Connection closed] peer_id: {}, connection_id: {}, endpoint: {:?}, num_established: {:?}", peer_id, connection_id, endpoint, num_established);
        }
        SwarmEvent::IncomingConnection { .. } => {}
        SwarmEvent::IncomingConnectionError { .. } => {}
        SwarmEvent::OutgoingConnectionError { .. } => {}
        SwarmEvent::NewListenAddr { .. } => {}
        SwarmEvent::ExpiredListenAddr { .. } => {}
        SwarmEvent::ListenerClosed { .. } => {}
        SwarmEvent::ListenerError { .. } => {}
        SwarmEvent::Dialing { .. } => {}
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

async fn delete_recipes(id: usize) -> Result<()> {
    let mut local_recipes = read_local_recipes().await?;
    local_recipes.retain(|r| r.id != id);
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
    fs::write(data_file(), &json).await?;
    Ok(())
}

async fn read_local_recipes() -> Result<Vec<Recipe>> {
    let content = fs::read(data_file()).await?;
    let result = serde_json::from_slice(&content)?;
    Ok(result)
}

fn respond_with_public_recipes(sender: mpsc::UnboundedSender<ListResponse>, receiver: String) {
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
