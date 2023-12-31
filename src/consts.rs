use libp2p::gossipsub::IdentTopic;
use libp2p::{identity, PeerId};
use once_cell::sync::Lazy;

pub const STORAGE_FILE_NAME: &str = "recipes.json";

pub const SYNC_LOG_FILE_NAME: &str = "sync.redb";

pub const RECIPES_STR: &str = "recipes";

pub const INIT_SYNC_STR: &str = "init-sync";

pub const BROADCAST_OPT_STR: &str = "broadcast-opt";

/// Key pair enables us to communicate securely with the rest of the network, making sure no one can impersonate
pub static KEYS: Lazy<identity::Keypair> = Lazy::new(identity::Keypair::generate_ed25519);

/// A unique identifier for a specific peer within the whole peer to peer network
///
/// Derive from a key pair to ensure its uniqueness
pub static PEER_ID: Lazy<PeerId> = Lazy::new(|| PeerId::from(KEYS.public()));

/// A Topic is a concept from Gossip, which is an implementation of libp2p’s pub/sub interface
pub static RECIPE_TOPIC: Lazy<IdentTopic> = Lazy::new(|| IdentTopic::new(RECIPES_STR));

/// A topic to announce init sync messages
pub static INIT_SYNC_TOPIC: Lazy<IdentTopic> = Lazy::new(|| IdentTopic::new(INIT_SYNC_STR));

/// A topic to broadcast the operations to all the other peers
pub static BROADCAST_OPT_TOPIC: Lazy<IdentTopic> = Lazy::new(|| IdentTopic::new(BROADCAST_OPT_STR));
