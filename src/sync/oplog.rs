use std::sync::OnceLock;

use hypercore::{Hypercore, HypercoreBuilder, Storage};
use libp2p::futures::executor::block_on;
use random_access_disk::RandomAccessDisk;
use tokio::sync::Mutex;

use crate::dir::base_dir;

pub static OPLOG_HANDLER: OnceLock<Mutex<OpLogHandler>> = OnceLock::new();

pub struct OpLogHandler {
    pub core: Hypercore<RandomAccessDisk>,
}

// let mut handler = OpLogHandler::global().await.lock().await;
// handler.core.append(b"123").await.unwrap();
// println!("{:?}", handler.core.info());

impl OpLogHandler {
    pub async fn global() -> &'static Mutex<Self> {
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
}
