use std::env;
use std::error::Error;
use std::time::Duration;

use libp2p::floodsub::Floodsub;
use libp2p::{mdns, noise, tcp, yamux, Swarm};
use log::{error, info};
use tokio::io::AsyncBufReadExt;
use tokio::sync::mpsc;

use crate::behaviour::RecipeBehaviour;
use crate::consts::{KEYS, PEER_ID, TOPIC};
use crate::dir::init_data;
use crate::handlers::{handle_create_recipe, handle_delete_recipe, handle_list_peers, handle_list_recipes, handle_publish_recipe, handle_swarm_event};
use crate::models::EventType;

mod behaviour;
mod consts;
mod handlers;
mod models;
mod dir;
mod sync;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env::set_var("RUST_LOG", "info");
    pretty_env_logger::init();

    info!("Peer Id: {}", PEER_ID.clone());
    init_data();

    let mut swarm = libp2p::SwarmBuilder::with_existing_identity(KEYS.clone())
        .with_tokio()
        .with_tcp(
            tcp::Config::default(),
            noise::Config::new,
            yamux::Config::default,
        )?
        .with_behaviour(|_key| RecipeBehaviour {
            flood_sub: Floodsub::new(*PEER_ID),
            mdns: mdns::tokio::Behaviour::new(mdns::Config::default(), KEYS.public().to_peer_id())
                .expect("can create mdns"),
        })?
        .with_swarm_config(|cfg| cfg.with_idle_connection_timeout(Duration::from_secs(5)))
        .build();
    Swarm::listen_on(
        &mut swarm,
        "/ip4/0.0.0.0/tcp/0"
            .parse()
            .expect("can get a local socket"),
    )
    .expect("swarm can be started");

    swarm.behaviour_mut().flood_sub.subscribe(TOPIC.clone());

    let (response_sender, mut response_rcv) = mpsc::unbounded_channel();
    let mut stdin = tokio::io::BufReader::new(tokio::io::stdin()).lines();
    loop {
        let evt = {
            tokio::select! {
                line = stdin.next_line() => Some(EventType::Input(line.expect("can get line").expect("can read line from stdin"))),
                response = response_rcv.recv() => Some(EventType::Response(response.expect("response exists"))),
                _ = handle_swarm_event(response_sender.clone(), &mut swarm) => None,
            }
        };

        if let Some(event) = evt {
            match event {
                EventType::Response(resp) => {
                    let json = serde_json::to_string(&resp).expect("can jsonify response");
                    swarm
                        .behaviour_mut()
                        .flood_sub
                        .publish(TOPIC.clone(), json.as_bytes());
                }
                EventType::Input(line) => match line.as_str() {
                    "ls p" => handle_list_peers(&mut swarm).await,
                    cmd if cmd.starts_with("create r") => handle_create_recipe(cmd).await,
                    cmd if cmd.starts_with("delete r") => handle_delete_recipe(cmd).await,
                    cmd if cmd.starts_with("publish r") => handle_publish_recipe(cmd).await,
                    cmd if cmd.starts_with("ls r") => handle_list_recipes(cmd, &mut swarm).await,
                    _ => error!("unknown command: {:?}", line),
                },
            }
        }
    }
}
