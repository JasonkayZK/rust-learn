use std::env;
use std::error::Error;
use std::time::Duration;

use libp2p::{mdns, noise, Swarm, tcp, yamux};
use libp2p::floodsub::{Floodsub, FloodsubEvent};
use libp2p::futures::StreamExt;
use libp2p::mdns::Event;
use libp2p::swarm::SwarmEvent;
use log::{error, info};
use tokio::io::AsyncBufReadExt;
use tokio::sync::mpsc;

use crate::behaviour::{RecipeBehaviour, RecipeBehaviourEvent};
use crate::consts::{KEYS, PEER_ID, TOPIC};
use crate::handlers::{
    handle_create_recipe, handle_list_peers, handle_list_recipes, handle_publish_recipe,
    respond_with_public_recipes,
};
use crate::models::{EventType, ListMode, ListRequest, ListResponse};

mod behaviour;
mod consts;
mod handlers;
mod models;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env::set_var("RUST_LOG", "debug");
    pretty_env_logger::init();

    info!("Peer Id: {}", PEER_ID.clone());
    let (response_sender, mut response_rcv) = mpsc::unbounded_channel();

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
                    cmd if cmd.starts_with("publish r") => handle_publish_recipe(cmd).await,
                    cmd if cmd.starts_with("ls r") => handle_list_recipes(cmd, &mut swarm).await,
                    _ => error!("unknown command: {:?}", line),
                },
            }
        }
    }
}

async fn handle_swarm_event(
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
        SwarmEvent::ConnectionEstablished { peer_id, connection_id, endpoint, num_established, .. } => {
            info!("[Connection established] peer_id: {}, connection_id: {}, endpoint: {:?}, num_established: {:?}", peer_id, connection_id, endpoint, num_established);
        }
        SwarmEvent::ConnectionClosed { peer_id, connection_id, endpoint, num_established, .. } => {
            info!("[Connection closed] peer_id: {}, connection_id: {}, endpoint: {:?}, num_established: {:?}", peer_id, connection_id, endpoint, num_established);
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
