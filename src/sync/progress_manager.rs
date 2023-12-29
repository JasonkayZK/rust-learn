use std::sync::OnceLock;

use libp2p::futures::executor::block_on;
use redb::{Database, Error, ReadableTable, TableDefinition};
use tokio::sync::Mutex;

use crate::dir::sync_log_file;

const TABLE: TableDefinition<&str, u64> = TableDefinition::new("sync_progress");

static PROGRESS_MANAGER: OnceLock<Mutex<ProgressManager>> = OnceLock::new();

pub struct ProgressManager {
    db: Database,
}

impl ProgressManager {
    async fn global() -> &'static Mutex<Self> {
        PROGRESS_MANAGER.get_or_init(|| {
            block_on(async {
                let db = Database::create(sync_log_file()).unwrap();
                Mutex::new(Self {
                    db,
                })
            })
        })
    }

    pub async fn get_key(k: &str) -> Result<Option<u64>, Error> {
        let db = &mut Self::global().await.lock().await.db;
        let read_txn = db.begin_read()?;
        let table = read_txn.open_table(TABLE)?;
        let ret = table.get(k)?.map(|val| val.value());
        Ok(ret)
    }

    pub async fn set_key(k: &str, v: u64) -> Result<(), Error> {
        let db = &mut Self::global().await.lock().await.db;
        let write_txn = db.begin_write()?;
        {
            let mut table = write_txn.open_table(TABLE)?;
            table.insert(k, v)?;
        }
        write_txn.commit()?;

        Ok(())
    }
}
