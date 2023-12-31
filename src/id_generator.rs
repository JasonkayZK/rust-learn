use std::sync::OnceLock;

use libp2p::futures::executor::block_on;
use sonyflake::Sonyflake;
use tokio::sync::Mutex;

static GLOBAL_ID_GENERATOR: OnceLock<Mutex<GlobalId>> = OnceLock::new();

pub struct GlobalId {
    generator: Sonyflake,
}

impl GlobalId {
    pub async fn next_id() -> u64 {
        Self::global()
            .await
            .lock()
            .await
            .generator
            .next_id()
            .unwrap()
    }

    async fn global() -> &'static Mutex<Self> {
        GLOBAL_ID_GENERATOR.get_or_init(|| {
            block_on(async {
                Mutex::new(Self {
                    generator: Sonyflake::new().unwrap(),
                })
            })
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::id_generator::GlobalId;

    #[tokio::test]
    async fn test_generate_id() {
        let id = GlobalId::next_id().await;
        println!("{}", id)
    }
}
