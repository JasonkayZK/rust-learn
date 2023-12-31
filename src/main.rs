use std::error::Error;
use std::u32;

use log::{error, info};
use tokio::io::AsyncBufReadExt;
use tokio::sync::mpsc;

use crate::consts::{BROADCAST_OPT_TOPIC, INIT_SYNC_TOPIC, PEER_ID, RECIPE_TOPIC};
use crate::dir::init_data;
use crate::handlers::{
    handle_create_recipe, handle_delete_recipe, handle_list_peers, handle_list_recipes,
    handle_list_sync_progress, handle_publish_recipe,
};
use crate::models::EventType;
use crate::swarm::handle_swarm_event;
use crate::swarm::handler::SwarmHandler;

mod behaviour;
mod consts;
mod dir;
mod handlers;
mod hlc;
mod id_generator;
mod logger;
mod models;
mod storage;
mod swarm;
mod sync;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    logger::init(Some(logger::LogLevel::Info));

    info!("Peer Id: {}", PEER_ID.clone());
    init_data();

    SwarmHandler::subscribe(&INIT_SYNC_TOPIC.clone())
        .await
        .unwrap();
    SwarmHandler::subscribe(&RECIPE_TOPIC.clone())
        .await
        .unwrap();
    SwarmHandler::subscribe(&BROADCAST_OPT_TOPIC.clone())
        .await
        .unwrap();

    let (response_sender, mut response_rcv) = mpsc::unbounded_channel();
    let mut stdin = tokio::io::BufReader::new(tokio::io::stdin()).lines();
    loop {
        let evt = {
            tokio::select! {
                line = stdin.next_line() => Some(EventType::Input(line.expect("can get line").expect("can read line from stdin"))),
                response = response_rcv.recv() => Some(EventType::Response(response.expect("response exists"))),
                _ = handle_swarm_event(response_sender.clone()) => None,
            }
        };

        if let Some(event) = evt {
            match event {
                EventType::Response(resp) => {
                    let json = serde_json::to_string(&resp).expect("can jsonify response");
                    SwarmHandler::publish(RECIPE_TOPIC.clone(), json.as_bytes())
                        .await
                        .unwrap();
                }
                EventType::Input(line) => match line.as_str() {
                    "ls p" => handle_list_peers().await,
                    "ls sp" => handle_list_sync_progress().await,
                    cmd if cmd.starts_with("create r") => handle_create_recipe(cmd).await,
                    cmd if cmd.starts_with("delete r") => handle_delete_recipe(cmd).await,
                    cmd if cmd.starts_with("publish r") => handle_publish_recipe(cmd).await,
                    cmd if cmd.starts_with("ls r") => handle_list_recipes(cmd).await,
                    _ => error!("unknown command: {:?}", line),
                },
            }
        }
    }
}
