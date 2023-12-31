use std::sync::OnceLock;

use hypercore::{AppendOutcome, Hypercore, HypercoreBuilder, HypercoreError, Info, Storage};
use libp2p::futures::executor::block_on;
use random_access_disk::RandomAccessDisk;
use tokio::sync::Mutex;

use crate::dir::op_log_dir;

static OPLOG_HANDLER: OnceLock<Mutex<OpLogHandler>> = OnceLock::new();

pub struct OpLogHandler {
    core: Hypercore<RandomAccessDisk>,
}

impl OpLogHandler {
    async fn global() -> &'static Mutex<Self> {
        OPLOG_HANDLER.get_or_init(|| {
            block_on(async {
                let storage = Storage::new_disk(&op_log_dir(), false).await.unwrap();
                let core = HypercoreBuilder::new(storage).build().await.unwrap();
                Mutex::new(Self { core })
            })
        })
    }

    pub async fn append(data: &[u8]) -> Result<AppendOutcome, HypercoreError> {
        let core = &mut Self::global().await.lock().await.core;
        core.append(data).await
    }

    pub async fn get_batch(indexes: &Vec<u64>) -> Result<Vec<Option<Vec<u8>>>, HypercoreError> {
        let core = &mut Self::global().await.lock().await.core;
        let mut ret = vec![];

        for idx in indexes {
            let v = core.get(*idx).await?;
            ret.push(v);
        }

        Ok(ret)
    }

    pub async fn get_info() -> Info {
        let core = &mut Self::global().await.lock().await.core;
        core.info()
    }
}
