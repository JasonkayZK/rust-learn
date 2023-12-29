use libp2p::{identity, PeerId};
use libp2p::gossipsub::IdentTopic;
use once_cell::sync::Lazy;

pub const STORAGE_FILE_NAME: &str = "recipes.json";

pub const SYNC_LOG_FILE_NAME: &str = "sync.redb";

/// Key pair enables us to communicate securely with the rest of the network, making sure no one can impersonate
pub static KEYS: Lazy<identity::Keypair> = Lazy::new(identity::Keypair::generate_ed25519);

/// A unique identifier for a specific peer within the whole peer to peer network
///
/// Derive from a key pair to ensure its uniqueness
pub static PEER_ID: Lazy<PeerId> = Lazy::new(|| PeerId::from(KEYS.public()));

/// A Topic is a concept from Floodsub, which is an implementation of libp2p’s pub/sub interface
pub static TOPIC: Lazy<IdentTopic> = Lazy::new(|| IdentTopic::new("recipes"));
