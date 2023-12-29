use std::sync::OnceLock;

use hypercore::{AppendOutcome, Hypercore, HypercoreBuilder, HypercoreError, Storage};
use libp2p::futures::executor::block_on;
use random_access_disk::RandomAccessDisk;
use tokio::sync::Mutex;

use crate::dir::base_dir;

static OPLOG_HANDLER: OnceLock<Mutex<OpLogHandler>> = OnceLock::new();

pub struct OpLogHandler {
    core: Hypercore<RandomAccessDisk>,
}

impl OpLogHandler {
    async fn global() -> &'static Mutex<Self> {
        OPLOG_HANDLER.get_or_init(|| {
            block_on(async {
                let storage = Storage::new_disk(&base_dir(), false).await.unwrap();
                let core = HypercoreBuilder::new(storage).build().await.unwrap();
                Mutex::new(Self {
                    core
                })
            })
        })
    }

    pub async fn append(data: &[u8]) -> Result<AppendOutcome, HypercoreError> {
        let core = &mut Self::global().await.lock().await.core;
        core.append(data).await
    }

    pub async fn get(idx: u64) -> Result<Option<Vec<u8>>, HypercoreError> {
        let core = &mut Self::global().await.lock().await.core;
        core.get(idx).await
    }
}
