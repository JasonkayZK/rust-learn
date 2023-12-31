use std::collections::hash_map::DefaultHasher;
use std::convert::TryFrom;
use std::hash::{Hash, Hasher};
use std::num::NonZeroU64;
use std::sync::OnceLock;

use libp2p::futures::executor::block_on;
use tokio::sync::Mutex;
use uhlc::{HLCBuilder, Timestamp, HLC, ID};

use crate::consts::PEER_ID;

static GLOBAL_HLC: OnceLock<Mutex<GlobalClock>> = OnceLock::new();

pub struct GlobalClock {
    clock: HLC,
}

impl GlobalClock {
    pub async fn timestamp() -> Timestamp {
        Self::global().await.lock().await.clock.new_timestamp()
    }

    pub async fn update_with_timestamp(timestamp: &Timestamp) {
        Self::global()
            .await
            .lock()
            .await
            .clock
            .update_with_timestamp(timestamp)
            .unwrap()
    }

    async fn global() -> &'static Mutex<Self> {
        GLOBAL_HLC.get_or_init(|| {
            block_on(async {
                let mut hasher = DefaultHasher::default();
                PEER_ID.to_string().hash(&mut hasher);
                let id = ID::from(NonZeroU64::try_from(hasher.finish()).unwrap());
                Mutex::new(Self {
                    clock: HLCBuilder::default().with_id(id).build(),
                })
            })
        })
    }
}
