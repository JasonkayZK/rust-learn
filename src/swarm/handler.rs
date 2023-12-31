use std::sync::{Arc, OnceLock};
use std::time::Duration;

use libp2p::futures::StreamExt;
use libp2p::gossipsub::{
    ConfigBuilder, IdentTopic, MessageAuthenticity, MessageId, PublishError, SubscriptionError,
    TopicHash,
};
use libp2p::swarm::{SwarmEvent, THandlerErr};
use libp2p::{gossipsub, mdns, noise, tcp, yamux, PeerId, Swarm};
use log::warn;
use tokio::sync::Mutex;

use crate::behaviour::{RecipeBehaviour, RecipeBehaviourEvent};
use crate::consts::KEYS;

static GLOBAL_SWARM_HANDLER: OnceLock<Arc<Mutex<SwarmHandler>>> = OnceLock::new();

pub struct SwarmHandler {
    swarm: Swarm<RecipeBehaviour>,
}

impl SwarmHandler {
    pub async fn has_node(peer_id: &PeerId) -> bool {
        let handler = SwarmHandler::global().await.clone();
        let swarm = &mut handler.lock().await.swarm;
        swarm.behaviour_mut().mdns.has_node(peer_id)
    }

    pub async fn remove_explicit_peer(peer_id: &PeerId) {
        let handler = SwarmHandler::global().await.clone();
        let swarm = &mut handler.lock().await.swarm;
        swarm.behaviour_mut().gossip.remove_explicit_peer(peer_id)
    }

    pub async fn add_explicit_peer(peer_id: &PeerId) {
        let handler = SwarmHandler::global().await.clone();
        let swarm = &mut handler.lock().await.swarm;
        swarm.behaviour_mut().gossip.add_explicit_peer(peer_id)
    }

    pub async fn select_next_some() -> SwarmEvent<RecipeBehaviourEvent, THandlerErr<RecipeBehaviour>>
    {
        let handler = SwarmHandler::global().await.clone();
        let swarm = &mut handler.lock().await.swarm;
        swarm.select_next_some().await
    }

    pub async fn discovered_nodes() -> Vec<PeerId> {
        let handler = SwarmHandler::global().await.clone();
        let swarm = &mut handler.lock().await.swarm;
        swarm
            .behaviour()
            .mdns
            .discovered_nodes()
            .copied()
            .collect::<Vec<PeerId>>()
    }

    pub async fn publish(
        topic: impl Into<TopicHash>,
        json: impl Into<Vec<u8>>,
    ) -> Result<MessageId, PublishError> {
        warn!("incoming publish");

        let handler = SwarmHandler::global().await.clone();
        let swarm = &mut handler.lock().await.swarm;
        swarm.behaviour_mut().gossip.publish(topic, json)
    }

    pub async fn unsubscribe(topic: &IdentTopic) -> Result<bool, PublishError> {
        let handler = SwarmHandler::global().await.clone();
        let swarm = &mut handler.lock().await.swarm;
        swarm.behaviour_mut().gossip.unsubscribe(topic)
    }

    pub async fn subscribe(topic: &IdentTopic) -> Result<bool, SubscriptionError> {
        let handler = SwarmHandler::global().await.clone();
        let swarm = &mut handler.lock().await.swarm;
        swarm.behaviour_mut().gossip.subscribe(topic)
    }

    async fn global() -> &'static Arc<Mutex<Self>> {
        GLOBAL_SWARM_HANDLER.get_or_init(|| {
            let mut swarm = libp2p::SwarmBuilder::with_existing_identity(KEYS.clone())
                .with_tokio()
                .with_tcp(
                    tcp::Config::default(),
                    noise::Config::new,
                    yamux::Config::default,
                )
                .unwrap()
                .with_behaviour(|_key| RecipeBehaviour {
                    gossip: gossipsub::Behaviour::new(
                        MessageAuthenticity::Signed(KEYS.clone()),
                        ConfigBuilder::default()
                            .idle_timeout(Duration::from_nanos(0))
                            .flood_publish(true)
                            .do_px()
                            .support_floodsub()
                            // .heartbeat_interval(Duration::from_secs(10)) // This is set to aid debugging by not cluttering the log space
                            .validation_mode(gossipsub::ValidationMode::Strict) // This sets the kind of message validation. The default is Strict (enforce message signing)
                            .build()
                            .unwrap(),
                    )
                    .unwrap(),
                    mdns: mdns::tokio::Behaviour::new(
                        mdns::Config::default(),
                        KEYS.public().to_peer_id(),
                    )
                    .expect("can create mdns"),
                })
                .unwrap()
                .build();

            Swarm::listen_on(
                &mut swarm,
                "/ip4/0.0.0.0/tcp/0"
                    .parse()
                    .expect("can get a local socket"),
            )
            .expect("swarm can be started");

            Arc::new(Mutex::new(Self { swarm }))
        })
    }
}
